use dotenv_codegen::dotenv;

mod db;
pub mod hash;
pub mod email;
mod initial_startup;
pub mod routes;
pub mod server;
mod jwt;

pub use db::*;
pub use initial_startup::*;

pub const DB_PATH: &str = dotenv!("DB_PATH");
pub const TEST_DB_PATH: &str = dotenv!("TEST_DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
