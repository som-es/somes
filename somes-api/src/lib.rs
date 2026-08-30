#![warn(clippy::unwrap_used)]

use std::sync::Arc;

use chrono::Local;
use combx::with_data::unique_topics::TopicsMapper;
use common_scrapes::eu_hemicycle::{HemicycleLayout, load_hemicycle};
use dotenvy_macro::dotenv;

pub mod cache_updater;
mod db;
pub mod email;
pub mod extractors;
pub use extractors::*;
mod filter_querying;
pub mod hash;
pub mod jwt;
pub mod refresh_views;
pub mod routes;
pub mod server;

pub use db::*;
pub use filter_querying::*;
pub use jwt::AuthError;
mod http_redirect;
pub mod meilisearch;
pub use http_redirect::*;
use once_cell::sync::Lazy;
pub mod parliament;
pub use parliament::*;
mod error;
pub use cache_updater::*;
pub use error::*;
use refresh_views::*;
use sqlx::PgPool;

use crate::db::redis_db::RedisHandle;

pub type Result<T> = std::result::Result<T, crate::error::GenericError>;

pub const USR_DATABASE_URL: &str = dotenv!("USR_DATABASE_URL");
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
pub const MAX_ENTRIES_PER_PAGE: &str = dotenv!("MAX_ENTRIES_PER_PAGE");
pub const STATIC_FRONTEND_PATH: &str = dotenv!("STATIC_FRONTEND_PATH");
pub const MEILISEARCH_URL: &str = dotenv!("MEILISEARCH_URL");
pub const MEILISEARCH_SECRET: &str = dotenv!("MEILISEARCH_SECRET");
pub const PRIVATE_KEY_PATH: &str = dotenv!("PRIVATE_KEY_PATH");
pub const PUBLIC_KEY_PATH: &str = dotenv!("PUBLIC_KEY_PATH");
pub const HTTP_PORT: &str = dotenv!("HTTP_PORT");
pub const HTTPS_PORT: &str = dotenv!("HTTPS_PORT");

pub static IS_PROD: Lazy<bool> = Lazy::new(|| is_prod());
pub static RESET_CACHE: Lazy<bool> = Lazy::new(|| reset_cache());

pub fn is_prod() -> bool {
    std::env::var("IS_PROD")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or_default()
}

pub fn reset_cache() -> bool {
    std::env::var("RESET_CACHE")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or_default()
}

static EMAIL_EXPIRATION_SECONDS: Lazy<usize> = Lazy::new(|| {
    dotenv!("EMAIL_EXPIRATION_SECONDS")
        .parse()
        .expect("Supplied email expiration (in seconds) is not a (usize) number!")
});

pub fn today_and_time() -> chrono::DateTime<Local> {
    chrono::Local::now()
}

pub fn today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisHandle,
    pub mcp_redis: RedisHandle,
    pub eu_redis: RedisHandle,
    pub dataservice_sqlx_pool: PgPool,
    pub eu_dataservice_sqlx_pool: PgPool,
    pub meilisearch_client: meilisearch_sdk::client::Client,
    pub eu_hemicycle: Arc<HemicycleLayout>,
    pub topics_mapper: Arc<TopicsMapper>,
}

impl AppState {
    pub fn new(
        redis: RedisHandle,
        eu_redis: RedisHandle,
        mcp_redis: RedisHandle,
        dataservice_sqlx_pool: PgPool,
        eu_dataservice_sqlx_pool: PgPool,
        meilisearch_client: meilisearch_sdk::client::Client,
        topics_mapper: TopicsMapper,
    ) -> AppState {
        AppState {
            redis,
            mcp_redis,
            eu_redis,
            dataservice_sqlx_pool,
            eu_dataservice_sqlx_pool,
            meilisearch_client,
            eu_hemicycle: Arc::new(load_hemicycle()),
            topics_mapper: Arc::new(topics_mapper),
        }
    }

    /// Returns the Postgres pool backing the given parliament. Austrian data
    /// lives in `DATASERVICE_URL`, EU data in `EU_DATASERVICE_URL`.
    pub fn pool(&self, parliament: Parliament) -> PgPool {
        match parliament {
            Parliament::At => self.dataservice_sqlx_pool.clone(),
            Parliament::Eu => self.eu_dataservice_sqlx_pool.clone(),
        }
    }

    pub fn redis(&self, parliament: Parliament) -> redis::aio::ConnectionManager {
        match parliament {
            Parliament::At => self.redis.connection.clone(),
            Parliament::Eu => self.eu_redis.connection.clone(),
        }
    }
}
