#![warn(clippy::unwrap_used)]

// use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use dotenvy_macro::dotenv;

pub mod dataservice;
mod db;
pub mod email;
pub mod hash;
mod initial_startup;
pub mod jwt;
pub mod routes;
pub mod server;
pub use db::*;
pub use initial_startup::*;
pub use jwt::AuthError;
mod http_redirect;
pub mod meilisearch;
pub use http_redirect::*;
use once_cell::sync::Lazy;
mod error;
pub mod views;
pub use error::*;

pub type Result<T> = std::result::Result<T, crate::error::GenericErrorResponse>;

pub const USR_DATABASE_URL: &str = dotenv!("USR_DATABASE_URL");
pub const DATASERVICE_URL: &str = dotenv!("DATASERVICE_URL");
pub const TEST_DB_PATH: &str = dotenv!("TEST_DB_PATH");
pub const SQL_SCHEMA_PATH: &str = dotenv!("SQL_SCHEMA_PATH");
pub const REDIS_DB: &str = dotenv!("REDIS_DB");
pub const VERIFICATION_SUBJECT: &str = dotenv!("VERIFICATION_SUBJECT");
pub const VERIFICATION_CONTENT: &str = dotenv!("VERIFICATION_CONTENT");
pub const API_ROOT: &str = dotenv!("API_ROOT");
pub const HOST_ADDR: &str = dotenv!("HOST_ADDR");
pub const LEGIS_INITS_PER_PAGE: &str = dotenv!("LEGIS_INITS_PER_PAGE");
pub const GOV_PROPS_PER_PAGE: &str = dotenv!("GOV_PROPS_PER_PAGE");
pub const DECREES_PER_PAGE: &str = dotenv!("DECREES_PER_PAGE");
pub const SPEECHES_PER_PAGE: &str = dotenv!("SPEECHES_PER_PAGE");
pub const ABSENCES_PER_PAGE: &str = dotenv!("ABSENCES_PER_PAGE");
pub const STATIC_FRONTEND_PATH: &str = dotenv!("STATIC_FRONTEND_PATH");
pub const MEILISEARCH_URL: &str = dotenv!("MEILISEARCH_URL");
pub const MEILISEARCH_SECRET: &str = dotenv!("MEILISEARCH_SECRET");
pub const PRIVATE_KEY_PATH: &str = dotenv!("PRIVATE_KEY_PATH");
pub const PUBLIC_KEY_PATH: &str = dotenv!("PUBLIC_KEY_PATH");
pub const HTTP_PORT: &str = dotenv!("HTTP_PORT");
pub const HTTPS_PORT: &str = dotenv!("HTTPS_PORT");

// pub type PostgresPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type PostgresPool = deadpool_diesel::postgres::Pool;

static EMAIL_EXPIRATION_SECONDS: Lazy<usize> = Lazy::new(|| {
    dotenv!("EMAIL_EXPIRATION_SECONDS")
        .parse()
        .expect("Supplied email expiration (in seconds) is not a (usize) number!")
});
// pub const EMAIL_EXPIRATION: u64 = ;

pub fn today_and_time() -> chrono::NaiveDateTime {
    chrono::Local::now().naive_local()
}

pub fn today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}
