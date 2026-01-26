use chrono::{NaiveTime, SecondsFormat};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp<T> {
    Eq(T),
    Ne(T),
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
    Cn(T),
    Ncn(T),
    Sw(T),
    In(T),
    Nin(T),
    Nsw(T),
    IsN(T),
    NIsN(T),
}

impl<T> FilterOp<T> {
    fn to_meilisearch_op(&self) -> String {
        match self {
            FilterOp::Eq(_) => format!("="),
            FilterOp::Gt(_) => format!(">"),
            FilterOp::Lt(_) => format!("<"),
            FilterOp::Cn(_) => format!("CONTAINS"),
            FilterOp::Ne(_) => format!("!="),
            FilterOp::Gte(_) => format!(">="),
            FilterOp::Lte(_) => format!("<="),
            FilterOp::Ncn(_) => format!("NOT CONTAINS"),
            FilterOp::Sw(_) => format!("STARTS WITH"),
            FilterOp::Nsw(_) => format!("NOT STARTS WITH"),
            FilterOp::In(_) => format!("IN"),
            FilterOp::Nin(_) => format!("NOT IN"),
            FilterOp::IsN(_) => format!("IS NULL"),
            FilterOp::NIsN(_) => format!("IS NOT NULL"),
        }
    }

    pub fn as_value(&self) -> &T {
        match self {
            FilterOp::Eq(val)
            | FilterOp::Gt(val)
            | FilterOp::Lt(val)
            | FilterOp::Cn(val)
            | FilterOp::Ne(val)
            | FilterOp::Gte(val)
            | FilterOp::Lte(val)
            | FilterOp::Ncn(val)
            | FilterOp::Sw(val)
            | FilterOp::In(val)
            | FilterOp::Nin(val)
            | FilterOp::IsN(val)
            | FilterOp::NIsN(val)
            | FilterOp::Nsw(val) => val,
        }
    }

    pub fn to_value(self) -> T {
        match self {
            FilterOp::Eq(val)
            | FilterOp::Gt(val)
            | FilterOp::Lt(val)
            | FilterOp::Cn(val)
            | FilterOp::Ne(val)
            | FilterOp::Gte(val)
            | FilterOp::Lte(val)
            | FilterOp::Ncn(val)
            | FilterOp::Sw(val)
            | FilterOp::In(val)
            | FilterOp::Nin(val)
            | FilterOp::IsN(val)
            | FilterOp::NIsN(val)
            | FilterOp::Nsw(val) => val,
        }
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Filterable {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value.to_utc().timestamp()),
        }
    }
}

impl ToFilterable for chrono::DateTime<chrono::Utc> {
    fn to_filterable(&self) -> Filterable {
        Filterable {
            value_as_string: format!("{:?}", self.timestamp()),
        }
    }
}

impl From<chrono::NaiveDate> for Filterable {
    fn from(value: chrono::NaiveDate) -> Self {
        Filterable {
            value_as_string: format!(
                "{:?}",
                value.and_time(NaiveTime::default()).and_utc().timestamp()
            ),
        }
    }
}

impl ToFilterable for chrono::NaiveDate {
    fn to_filterable(&self) -> Filterable {
        Filterable {
            value_as_string: format!(
                "{:?}",
                self.and_time(NaiveTime::default()).and_utc().timestamp()
            ),
        }
    }
}

impl From<String> for Filterable {
    fn from(value: String) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value),
        }
    }
}

impl ToFilterable for String {
    fn to_filterable(&self) -> Filterable {
        Filterable {
            value_as_string: format!("{:?}", self),
        }
    }
}

impl From<&str> for Filterable {
    fn from(value: &str) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value),
        }
    }
}

impl ToFilterable for &String {
    fn to_filterable(&self) -> Filterable {
        Filterable {
            value_as_string: format!("{:?}", self),
        }
    }
}

macro_rules! impl_filterable {
    ($($val:ident),*) => {
        $(
            impl From<$val> for Filterable {
                fn from(value: $val) -> Self {
                    Filterable {
                        value_as_string: value.to_string()
                    }
                }
            }

            impl ToFilterable for $val {
                fn to_filterable(&self) -> Filterable {
                    Filterable {
                        value_as_string: self.to_string()
                    }
                }
            }
        )*
    };
}

impl_filterable! {
    f32, f64, i8, i16, i32, i64, i128,
    isize, u8, u16, u32, u64, u128, usize, bool
}

impl<T: Into<Filterable> + std::fmt::Debug> From<Vec<T>> for Filterable {
    fn from(value: Vec<T>) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value),
        }
    }
}

impl<T: ToFilterable + std::fmt::Debug> ToFilterable for Vec<T> {
    fn to_filterable(&self) -> Filterable {
        Filterable {
            value_as_string: format!("{:?}", self),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Filterable {
    value_as_string: String,
}

pub trait ToFilterable {
    fn to_filterable(&self) -> Filterable;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FilterArgument {
    filter_attribute_path: String,
    filter_op: FilterOp<Filterable>,
}

pub trait ToFilterArgument {
    fn to_filter(&self, attribute_path: &str) -> Option<FilterArgument>;
}

impl<T: ToFilterArgument> ToFilterArgument for Option<T> {
    fn to_filter(&self, attribute_path: &str) -> Option<FilterArgument> {
        match self {
            Some(val) => val.to_filter(attribute_path),
            None => None,
        }
    }
}

impl<T: ToFilterable> ToFilterArgument for FilterOp<T> {
    fn to_filter(&self, attribute_path: &str) -> Option<FilterArgument> {
        let filter_op = match self {
            FilterOp::Eq(val) => FilterOp::Eq(val.to_filterable()),
            FilterOp::Gt(val) => FilterOp::Gt(val.to_filterable()),
            FilterOp::Lt(val) => FilterOp::Lt(val.to_filterable()),
            FilterOp::Cn(val) => FilterOp::Cn(val.to_filterable()),
            FilterOp::Ne(val) => FilterOp::Ne(val.to_filterable()),
            FilterOp::Gte(val) => FilterOp::Gte(val.to_filterable()),
            FilterOp::Lte(val) => FilterOp::Lte(val.to_filterable()),
            FilterOp::Ncn(val) => FilterOp::Ncn(val.to_filterable()),
            FilterOp::Sw(val) => FilterOp::Sw(val.to_filterable()),
            FilterOp::In(val) => FilterOp::In(val.to_filterable()),
            FilterOp::Nin(val) => FilterOp::Nin(val.to_filterable()),
            FilterOp::NIsN(val) => FilterOp::NIsN(val.to_filterable()),
            FilterOp::IsN(val) => FilterOp::IsN(val.to_filterable()),
            FilterOp::Nsw(val) => FilterOp::Nsw(val.to_filterable()),
        };

        Some(FilterArgument {
            filter_attribute_path: attribute_path.to_string(),
            filter_op,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombineOp {
    #[default]
    And,
    Or,
}

impl CombineOp {
    pub fn to_meilisearch_op(&self) -> &'static str {
        match self {
            CombineOp::And => "AND",
            CombineOp::Or => "OR",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterOptions {
    pub combine_op: CombineOp,
    pub prefix: Option<String>,
}

pub fn to_meilisearch_filter(filter_args: &[Option<FilterArgument>]) -> String {
    to_meilisearch_filter_with_ops(filter_args, &FilterOptions::default())
}

pub fn to_meilisearch_filters(
    filter_args: &[Option<FilterArgument>],
    options: &FilterOptions,
) -> Vec<String> {
    filter_args
        .iter()
        .flatten()
        .map(|filter_arg| {
            let prefix = if let Some(prefix) = &options.prefix {
                format!("{}.", prefix)
            } else {
                "".to_string()
            };
            format!(
                "{prefix}{attribute} {op} {value}",
                attribute = filter_arg.filter_attribute_path,
                op = filter_arg.filter_op.to_meilisearch_op(),
                value = filter_arg.filter_op.as_value().value_as_string
            )
        })
        .collect::<Vec<String>>()
}

pub fn to_meilisearch_filter_with_ops(
    filter_args: &[Option<FilterArgument>],
    options: &FilterOptions,
) -> String {
    format!("({})", to_meilisearch_filters(filter_args, options)
        .join(&format!(" {} ", options.combine_op.to_meilisearch_op())))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{FilterOp, Filterable, ToFilterArgument, to_meilisearch_filter};

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct InnerGpFilter {
        pub legis_period: Option<FilterOp<Vec<String>>>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct TopicFilter {
        pub topic: FilterOp<String>,
    }

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct DecreeFilter2 {
        pub legis_period: Option<FilterOp<String>>,
        pub gov_officials: Option<FilterOp<Vec<i32>>>,
        pub inner_gp: Option<InnerGpFilter>,
        pub topics: Option<Vec<TopicFilter>>,
    }

    #[test]
    fn it_works() {
        let x: Filterable = "string".to_string().into();
        let x: Filterable = 4i32.into();
        let x: Filterable = vec!["hallo", "hi"].into();
        let res = format!("servas_oida = {}", x.value_as_string);
        dbg!(res);

        let query_str =
            "legis_period[eq]=XXVIII&gov_officials[in][0]=1&inner_gp[legis_period][in][0]=XXVII";
        let data = serde_qs::from_str::<DecreeFilter2>(query_str).unwrap();
        dbg!(&data);
        let filter = [
            data.gov_officials.to_filter("gov_official_id"),
            data.legis_period.to_filter("gp"),
        ];
        let filter = to_meilisearch_filter(&filter);
        dbg!(filter);

        let query_str = "topics[0][topic][sw]=Education&topics[1][topic][sw]=Health";
        let data = serde_qs::from_str::<DecreeFilter2>(query_str).unwrap();
        dbg!(&data);

        // let res = data.
        // dbg!(res);
        // let filter = DecreeFilter2 {
        //     legis_period: Some(crate::FilterOp::Eq("XXVIII".into())),
        //     gov_officials: None,
        // };

        // filter.legis_period.
    }
}
