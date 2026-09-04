#![warn(clippy::unwrap_used)]

use std::sync::Arc;

use chrono::Local;
use combx::with_data::unique_topics::TopicsMapper;
use common_scrapes::eu_hemicycle::{HemicycleLayout, load_hemicycle};

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

pub fn env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("environment variable {name} must be set"))
}

pub static USR_DATABASE_URL: Lazy<String> = Lazy::new(|| env_var("USR_DATABASE_URL"));
pub static REDIS_DB: Lazy<String> = Lazy::new(|| env_var("REDIS_DB"));
pub static VERIFICATION_SUBJECT: Lazy<String> = Lazy::new(|| env_var("VERIFICATION_SUBJECT"));
pub static VERIFICATION_CONTENT: Lazy<String> = Lazy::new(|| env_var("VERIFICATION_CONTENT"));
pub static API_ROOT: Lazy<String> = Lazy::new(|| env_var("API_ROOT"));
pub static HOST_ADDR: Lazy<String> = Lazy::new(|| env_var("HOST_ADDR"));
pub static LEGIS_INITS_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("LEGIS_INITS_PER_PAGE"));
pub static GOV_PROPS_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("GOV_PROPS_PER_PAGE"));
pub static DELEGATE_QUESTIONS_PER_PAGE: Lazy<String> =
    Lazy::new(|| std::env::var("DELEGATE_QUESTIONS_PER_PAGE").unwrap_or_else(|_| "16".into()));
pub static DECREES_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("DECREES_PER_PAGE"));
pub static SPEECHES_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("SPEECHES_PER_PAGE"));
pub static ABSENCES_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("ABSENCES_PER_PAGE"));
pub static MAX_ENTRIES_PER_PAGE: Lazy<String> = Lazy::new(|| env_var("MAX_ENTRIES_PER_PAGE"));
pub static STATIC_FRONTEND_PATH: Lazy<String> = Lazy::new(|| env_var("STATIC_FRONTEND_PATH"));
pub static MEILISEARCH_URL: Lazy<String> = Lazy::new(|| env_var("MEILISEARCH_URL"));
pub static MEILISEARCH_SECRET: Lazy<String> = Lazy::new(|| env_var("MEILISEARCH_SECRET"));
pub static PRIVATE_KEY_PATH: Lazy<String> = Lazy::new(|| env_var("PRIVATE_KEY_PATH"));
pub static PUBLIC_KEY_PATH: Lazy<String> = Lazy::new(|| env_var("PUBLIC_KEY_PATH"));
pub static HTTP_PORT: Lazy<String> = Lazy::new(|| env_var("HTTP_PORT"));
pub static HTTPS_PORT: Lazy<String> = Lazy::new(|| env_var("HTTPS_PORT"));

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
    env_var("EMAIL_EXPIRATION_SECONDS")
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
