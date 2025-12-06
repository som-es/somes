use sqlx::{Encode, Postgres, Type};

pub trait Filterable<'a, What>: Send + Sync {
    fn should_return_any(&self) -> bool;
    fn has_value(&self) -> bool {
        true
    }
    fn to_query_part(&self, sql_column_name: &str, idx: usize) -> String;
    fn bind(
        &'a self,
        bind_on_query: sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments>;
}
#[derive(Debug, Clone, Copy)]
pub struct Nullable<T>(pub Option<Option<T>>);

impl<'a, What, T: Clone + Send + Sync + Encode<'a, Postgres> + Type<Postgres>> Filterable<'a, What>
    for Nullable<T>
{
    fn should_return_any(&self) -> bool {
        self.0.is_none()
    }

    fn to_query_part(&self, sql_column_name: &str, idx: usize) -> String {
        match &self.0 {
            Some(inner) => {
                let and = if idx == 0 { " " } else { " and " };
                match inner {
                    Some(_val) => {
                        format!("{and}{sql_column_name} = ${}", idx + 1)
                    }
                    None => format!("{and}{sql_column_name} is null"),
                }
            }
            None => " ".to_string(),
        }
    }

    fn bind(
        &'a self,
        bind_on_query: sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments> {
        bind_on_query.bind(self.0.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Manual(pub &'static str);

impl Manual {
    // pub fn to_filter_arg() -> FilterArgument<>
}

impl<'a, What> Filterable<'a, What> for Manual {
    fn should_return_any(&self) -> bool {
        false
    }
    fn has_value(&self) -> bool {
        false
    }

    fn to_query_part(&self, sql_column_name: &str, idx: usize) -> String {
        let and = if idx == 0 { " " } else { " and " };
        format!("{and}{sql_column_name}")
    }

    fn bind(
        &'a self,
        bind_on_query: sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments> {
        bind_on_query.bind(self.0)
    }
}

impl<'a, What, T: Clone + 'static + Encode<'a, Postgres> + Type<Postgres> + Send + Sync>
    Filterable<'a, What> for Option<T>
{
    fn to_query_part(&self, sql_column_name: &str, idx: usize) -> String {
        match self {
            Some(_) => {
                let and = if idx == 0 { " " } else { " and " };

                format!("{and}{sql_column_name} = ${}", idx + 1)
            }
            None => " ".to_string(),
        }
    }

    fn bind(
        &'a self,
        bind_on_query: sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'a, sqlx::Postgres, What, sqlx::postgres::PgArguments> {
        bind_on_query.bind(self.as_ref())
    }

    fn should_return_any(&self) -> bool {
        self.is_none()
    }
}

pub struct FilterArgument<'a, 'b, What> {
    value: &'a dyn Filterable<'b, What>,
    sql_column_name: &'static str,
}

impl<'a, 'b, What> FilterArgument<'a, 'b, What> {
    pub fn new(value: &'a dyn Filterable<'b, What>, column: &'static str) -> Self {
        FilterArgument {
            value,
            sql_column_name: column,
        }
    }
}

pub fn count_filter<What>(filters: &[FilterArgument<'_, '_, What>]) -> usize {
    filters
        .iter()
        .filter(|x| !x.value.should_return_any())
        .count()
}

pub fn build_filter<What>(filters: &[FilterArgument<'_, '_, What>]) -> String {
    let filter_part: String = filters
        .iter()
        .filter(|x| !x.value.should_return_any())
        .enumerate()
        .map(|(idx, x)| x.value.to_query_part(x.sql_column_name, idx))
        .collect();
    if filter_part.is_empty() {
        return " true ".to_string()
    }
    filter_part
}

pub fn bind_values<'a, 'b, What>(
    mut query: sqlx::query::QueryAs<'b, sqlx::Postgres, What, sqlx::postgres::PgArguments>,
    filters: &[FilterArgument<'b, 'b, What>],
) -> sqlx::query::QueryAs<'b, sqlx::Postgres, What, sqlx::postgres::PgArguments> {
    for filter in filters {
        if filter.value.should_return_any() || !filter.value.has_value() {
            continue;
        }
        query = filter.value.bind(query)
    }
    query
}

pub trait IntoFilterArgument<'b> {
    fn into_filter_arg<'a, What>(
        &'b self,
    ) -> FilterArgument<'a, 'b, What> {
        self.with_sql_column("")
    }
    
    fn with_sql_column<'a, What>(
        &'b self,
        sql_column: &'static str,
    ) -> FilterArgument<'a, 'b, What>;
}

impl<'b, T: Clone + 'static + Encode<'b, Postgres> + Type<Postgres> + Send + Sync>
    IntoFilterArgument<'b> for Option<T>
{
    fn with_sql_column<'a, What>(
        &'b self,
        sql_column: &'static str,
    ) -> FilterArgument<'a, 'b, What> {
        FilterArgument::new(self, sql_column)
    }
}

impl<'b> IntoFilterArgument<'b> for Manual {
    fn with_sql_column<'a, What>(
        &'b self,
        _sql_column: &'static str,
    ) -> FilterArgument<'a, 'b, What> {
        FilterArgument::new(self, self.0)
    }
}

impl<'b, T: Clone + 'static + Encode<'b, Postgres> + Type<Postgres> + Send + Sync>
    IntoFilterArgument<'b> for Nullable<T>
{
    fn with_sql_column<'a, What>(
        &'b self,
        sql_column: &'static str,
    ) -> FilterArgument<'a, 'b, What> {
        FilterArgument::new(self, sql_column)
    }
}


#[cfg(test)]
mod tests {
    use dataservice::connect_pg;
    use sqlx::{prelude::FromRow, Postgres};

    use crate::routes::statistics::filtering::{bind_values, IntoFilterArgument};

    pub struct CallToOrderFilter {
        legis_period: Option<String>,
        gender: Option<String>,
        party: Option<String>,
    }

    #[derive(PartialEq, Debug, Clone, FromRow)]
    pub struct CallToOrdersForDelegate {
        delegate_name: String,
        delegate_party: String,
        delegate_gender: String,
        total_order_calls: i64,
    }

    #[tokio::test]
    async fn test_build_filter() {
        use crate::routes::statistics::filtering::build_filter;

        // von api als input
        let filter = CallToOrderFilter {
            legis_period: Some("XXV".to_string()),
            gender: Some("m".to_string()),
            party: Some("FPÖ".to_string()),
        };

        // irgendwie alle fields zu so ding da machen (array)
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.gender.with_sql_column("ds.gender");
        let filter_arg2 = filter.party.with_sql_column("ds.party");
        let filter_arg3 = Some("nr").with_sql_column("council");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        // daraus kriegst du dann einen string der den filter in der query represented
        let filter = build_filter(&filters);

        let query = format!(
            "
            SELECT 
                ds.name AS delegate_name,
                ds.party AS delegate_party,
                ds.gender AS delegate_gender,
                COUNT(cto.id) AS total_order_calls
            FROM 
                call_to_order cto
            JOIN 
                delegates ds ON cto.receiver_id = ds.id
            JOIN 
                plenar_infos pf ON pf.id = cto.plenar_id
            WHERE 
                {filter}
            GROUP BY 
                ds.id, ds.name, ds.party, ds.gender
            ORDER BY 
                total_order_calls DESC;

        "
        );
        println!("{query}");

        let mut filtered_query = sqlx::query_as::<Postgres, CallToOrdersForDelegate>(&query);
        // setzt dann die filter werte auf die query parameter ok??
        filtered_query = bind_values(filtered_query, &filters);

        // TIM DAS NICHT MACHEN - &PgPool bekommst du von der somes api als parameter!!!!
        let pg = connect_pg().await;
        let res = filtered_query.fetch_all(&pg).await.unwrap();
        println!("res: {res:?}")
    }
}
