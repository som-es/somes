use chrono::SecondsFormat;
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
            | FilterOp::Nsw(val) => val,
        }
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Filterable {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        Filterable {
            value_as_string: format!(
                "{}",
                value.to_utc().to_rfc3339_opts(SecondsFormat::Micros, true)
            ),
        }
    }
}
impl From<chrono::NaiveDate> for Filterable {
    fn from(value: chrono::NaiveDate) -> Self {
        Filterable {
            value_as_string: format!("{}", value.to_string()),
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

impl From<&str> for Filterable {
    fn from(value: &str) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value),
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
        )*
    };
}

impl_filterable! {
    f32, f64, i8, i16, i32, i64, i128,
    isize, u8, u16, u32, u64, u128, usize
}

impl<T: Into<Filterable> + std::fmt::Debug> From<Vec<T>> for Filterable {
    fn from(value: Vec<T>) -> Self {
        Filterable {
            value_as_string: format!("{:?}", value),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Filterable {
    value_as_string: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FilterArgument {
    filter_attribute_path: String,
    filter_op: FilterOp<Filterable>,
}

pub trait IntoFilterArgument {
    fn into_filter(self, attribute_path: &str) -> Option<FilterArgument>;
}

impl<T: IntoFilterArgument> IntoFilterArgument for Option<T> {
    fn into_filter(self, attribute_path: &str) -> Option<FilterArgument> {
        match self {
            Some(val) => val.into_filter(attribute_path),
            None => None,
        }
    }
}

impl<T: Into<Filterable>> IntoFilterArgument for FilterOp<T> {
    fn into_filter(self, attribute_path: &str) -> Option<FilterArgument> {
        let filter_op = match self {
            FilterOp::Eq(val) => FilterOp::Eq(val.into()),
            FilterOp::Gt(val) => FilterOp::Gt(val.into()),
            FilterOp::Lt(val) => FilterOp::Lt(val.into()),
            FilterOp::Cn(val) => FilterOp::Cn(val.into()),
            FilterOp::Ne(val) => FilterOp::Ne(val.into()),
            FilterOp::Gte(val) => FilterOp::Gte(val.into()),
            FilterOp::Lte(val) => FilterOp::Lte(val.into()),
            FilterOp::Ncn(val) => FilterOp::Ncn(val.into()),
            FilterOp::Sw(val) => FilterOp::Sw(val.into()),
            FilterOp::In(val) => FilterOp::In(val.into()),
            FilterOp::Nin(val) => FilterOp::Nin(val.into()),
            FilterOp::Nsw(val) => FilterOp::Nsw(val.into()),
        };

        Some(FilterArgument {
            filter_attribute_path: attribute_path.to_string(),
            filter_op,
        })
    }
}

pub fn to_meilisearch_filter(filter_args: &[Option<FilterArgument>]) -> String {
    filter_args
        .iter()
        .flatten()
        .map(|filter_arg| {
            format!(
                "{attribute} {op} {value}",
                attribute = filter_arg.filter_attribute_path,
                op = filter_arg.filter_op.to_meilisearch_op(),
                value = filter_arg.filter_op.as_value().value_as_string
            )
        })
        .collect::<Vec<String>>()
        .join(" AND ")
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{FilterOp, Filterable, IntoFilterArgument, to_meilisearch_filter};

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct DecreeFilter2 {
        pub legis_period: Option<FilterOp<String>>,
        pub gov_officials: Option<FilterOp<Vec<i32>>>,
    }

    #[test]
    fn it_works() {
        let x: Filterable = "string".to_string().into();
        let x: Filterable = 4i32.into();
        let x: Filterable = vec!["hallo", "hi"].into();
        let res = format!("servas_oida = {}", x.value_as_string);
        dbg!(res);

        let query_str = "legis_period[eq]=XXVIII&gov_officials[in][0]=1";

        let data = serde_qs::from_str::<DecreeFilter2>(query_str).unwrap();
        dbg!(&data);
        let filter = [
            data.gov_officials.into_filter("gov_official_id"),
            data.legis_period.into_filter("gp"),
        ];
        let filter = to_meilisearch_filter(&filter);
        dbg!(filter);

        // let res = data.
        // dbg!(res);
        // let filter = DecreeFilter2 {
        //     legis_period: Some(crate::FilterOp::Eq("XXVIII".into())),
        //     gov_officials: None,
        // };

        // filter.legis_period.
    }
}
