use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use chrono::NaiveDate;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use sqlx::PgPool;

pub mod model;
pub use model::*;

use crate::{AppState, today};

#[cfg(not(debug_assertions))]
use redis::Commands;

pub mod redis_db {
    pub const AT_DB: u32 = 0;
    pub const EU_DB: u32 = 1;
    pub const MCP_DB: u32 = 255;
}

pub async fn get_json_cache<T: DeserializeOwned>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
) -> Option<T> {
    #[cfg(debug_assertions)]
    {
        let _ = key;
        None
    }
    #[cfg(not(debug_assertions))]
    {
        serde_json::from_str(&redis_client.get::<&str, String>(key).await.ok()?).ok()
    }
}

pub async fn set_json_cache_no_expire<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
) -> Option<()> {
    redis_client
        .set::<_, _, ()>(key, serde_json::to_string(value).ok()?)
        .await
        .ok()?;
    Some(())
}

pub async fn set_json_cache_secs<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
    seconds: i64,
) -> Option<()> {
    redis_client
        .set::<_, _, ()>(key, serde_json::to_string(value).ok()?)
        .await
        .ok()?;
    redis_client.expire::<_, ()>(key, seconds).await.ok()?;
    Some(())
}

pub async fn set_json_cache<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
) -> Option<()> {
    set_json_cache_secs(redis_client, key, value, 1200).await
}

pub async fn set_json_cache_with_relevance<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
    date: NaiveDate,
) -> Option<()> {
    let dur = today() - date;
    let seconds = ((dur.num_days() as f32).powf(1.2) as i64 * 30)
        .min(60 * 60 * 24 * 2)
        .max(500);
    log::trace!(
        "seconds cached: {seconds}, (days: {})",
        seconds / (60 * 60 * 24)
    );
    set_json_cache_secs(redis_client, key, value, seconds).await
}

pub struct McpRedisConnection(pub redis::aio::MultiplexedConnection);

impl FromRequestParts<AppState> for McpRedisConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let pool = &state.mcp_redis_client;
        let conn = pool
            .get_multiplexed_async_connection()
            .await
            .map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub struct RedisConnection(pub redis::aio::MultiplexedConnection);
// #[async_trait]
impl FromRequestParts<AppState> for RedisConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let parliament = parts
            .extensions
            .get::<combx::Parliament>()
            .copied()
            .unwrap_or_default();

        let pool = state.redis(parliament);
        let conn = pool
            .get_multiplexed_async_connection()
            .await
            .map_err(internal_error)?;

        Ok(Self(conn))
    }
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(app_state: &AppState) -> redis::Client {
        app_state.redis_client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.dataservice_sqlx_pool.clone()
    }
}

// Parliament-aware: `/api/eu/...` requests carry a `Parliament::Eu` extension
// (injected by the route nest in `crate::server`) and get the EU Postgres pool;
// everything else falls back to the Austrian pool.
impl FromRequestParts<AppState> for PgPoolConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let parliament = parts
            .extensions
            .get::<combx::Parliament>()
            .copied()
            .unwrap_or_default();
        Ok(Self(state.pool(parliament)))
    }
}

pub struct PgPoolConnection(pub PgPool);

pub struct AtPgPoolConnection(pub PgPool);

impl FromRequestParts<AppState> for AtPgPoolConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.dataservice_sqlx_pool.clone()))
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
