use dotenv_codegen::dotenv;

mod db;
pub mod hash;
mod initial_startup;
pub mod routes;
pub mod server;

pub use db::*;
pub use initial_startup::*;

pub const DB_PATH: &str = dotenv!("DB_PATH");
pub const TEST_DB_PATH: &str = dotenv!("TEST_DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
