use dotenv_codegen::dotenv;

mod db;
pub mod hash;
mod initial_startup;
pub mod routes;
pub mod server;

pub use initial_startup::initial_startup;
use once_cell::sync::Lazy;

pub const DB_PATH: &str = dotenv!("DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
pub const MIN_PASSWORD_LEN: Lazy<usize> = Lazy::new(|| {
    let x: &str = dotenv!("MIN_PASSWORD_LEN");
    x.parse::<usize>().expect("The value provided in the .env is not a number!")
});
