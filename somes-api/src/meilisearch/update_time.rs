use chrono::Utc;
use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::meilisearch::Index;

pub async fn update_update_time_of_index(
    redis_con: &mut MultiplexedConnection,
    index: &Index,
) -> redis::RedisResult<()> {
    let now = Utc::now().to_rfc3339();
    let _: () = redis_con
        .set(format!("meilisearch_last_update_time_{}", index), now)
        .await?;
    Ok(())
}

pub async fn get_update_time_of_index(
    redis_con: &mut MultiplexedConnection,
    index: &Index,
) -> redis::RedisResult<chrono::DateTime<Utc>> {
    let time: String = redis_con
        .get(format!("meilisearch_last_update_time_{}", index.as_str()))
        .await?;
    let datetime = chrono::DateTime::parse_from_rfc3339(&time)
        .map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::UnexpectedReturnType,
                "Failed to parse datetime",
                e.to_string(),
            ))
        })?
        .with_timezone(&Utc);
    Ok(datetime)
}
