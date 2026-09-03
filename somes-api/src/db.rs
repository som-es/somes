use chrono::NaiveDate;
use redis::AsyncCommands;
use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};

pub mod model;
pub use model::*;

use crate::today;

#[cfg(not(debug_assertions))]
use redis::Commands;

pub mod redis_db {
    use redis::aio::ConnectionManager;

    pub const AT_DB: u32 = 0;
    pub const EU_DB: u32 = 1;
    pub const MCP_DB: u32 = 255;

    #[derive(Clone)]
    pub struct RedisHandle {
        pub client: redis::Client,
        pub connection: ConnectionManager,
    }

    impl RedisHandle {
        pub async fn new(client: redis::Client) -> redis::RedisResult<Self> {
            Ok(Self {
                connection: ConnectionManager::new(client.clone()).await?,
                client,
            })
        }
    }
}

pub async fn get_json_cache<T: DeserializeOwned>(
    _redis_client: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    key: &str,
) -> Option<T> {
    #[cfg(debug_assertions)]
    {
        let _ = key;
        None
    }
    #[cfg(not(debug_assertions))]
    {
        serde_json::from_str(&_redis_client.get::<&str, String>(key).await.ok()?).ok()
    }
}

pub async fn set_json_cache_no_expire<T: Serialize>(
    redis_client: &mut (impl redis::aio::ConnectionLike + Send + Sync),
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
    redis_client: &mut (impl redis::aio::ConnectionLike + Send + Sync),
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
    redis_client: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    key: &str,
    value: &T,
) -> Option<()> {
    set_json_cache_secs(redis_client, key, value, 1200).await
}

pub async fn set_json_cache_with_relevance<T: Serialize>(
    redis_client: &mut (impl redis::aio::ConnectionLike + Send + Sync),
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

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
