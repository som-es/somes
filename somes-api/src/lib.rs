use dotenv_codegen::dotenv;

mod db;
mod initial_startup;
pub mod server;

pub use initial_startup::initial_startup;

pub const DB_PATH: &str = dotenv!("DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
