pub mod errors;
pub mod password;
mod routes;
mod structs;
pub mod time;

use once_cell::sync::Lazy;
pub use routes::*;
pub use structs::*;

pub trait ToCompositeType {
    fn field_orders() -> Vec<&'static str> {
        vec![]
    }
    fn type_name() -> &'static str;
    fn to_sql_create_composite_type() -> String;
}

pub static VERIFY_ID_INVALID_HOURS: Lazy<usize> = Lazy::new(|| {
    let verify_id_invalid_hours: String =
        std::env::var("VERIFY_ID_INVALID_HOURS").unwrap_or("24".into());
    verify_id_invalid_hours
        .parse::<usize>()
        .expect("The value provided for VERIFY_ID_INVALID_HOURS in the .env is not a number!")
});
