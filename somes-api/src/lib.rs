use dotenv_codegen::dotenv;

pub mod dataservice;
mod db;
pub mod email;
pub mod hash;
mod initial_startup;
mod jwt;
pub mod routes;
pub mod server;

pub use db::*;
pub use initial_startup::*;

pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");
pub const DATASERVICE_URL: &str = dotenv!("DATASERVICE_URL");
pub const TEST_DB_PATH: &str = dotenv!("TEST_DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
pub const REDIS_DB: &str = dotenv!("REDIS_DB");

pub fn today_and_time() -> chrono::NaiveDateTime {
    chrono::Local::now().naive_local()
}
