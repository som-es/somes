use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use chrono::NaiveDate;
use redis::{aio::MultiplexedConnection, AsyncCommands, Commands};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::PgPool;

pub mod model;
pub use model::*;

use crate::{server::AppState, today};

pub async fn get_json_cache<T: DeserializeOwned>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
) -> Option<T> {
    #[cfg(debug_assertions)]
    {
        None
    }
    #[cfg(not(debug_assertions))]
    serde_json::from_str(&redis_client.get::<&str, String>(key).await.ok()?).ok()
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

pub struct RedisConnection(pub redis::aio::MultiplexedConnection);

// #[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    redis::Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = redis::Client::from_ref(state);
        let conn = pool
            .get_multiplexed_async_connection()
            .await
            .map_err(internal_error)?;

        Ok(Self(conn))
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.dataservice_sqlx_pool.clone()
    }
}

// #[async_trait]
impl<S> FromRequestParts<S> for PgPoolConnection
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        Ok(Self(pool))
    }
}

pub struct PgPoolConnection(pub PgPool);

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
