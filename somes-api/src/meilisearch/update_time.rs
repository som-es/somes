use chrono::Utc;
use combx::Parliament;
use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::meilisearch::Index;

fn update_time_key(parliament: Parliament, index: &Index) -> String {
    format!(
        "meilisearch_last_update_time_{}",
        index.uid(parliament)
    )
}

pub async fn update_update_time_of_index(
    redis_con: &mut MultiplexedConnection,
    parliament: Parliament,
    index: &Index,
) -> redis::RedisResult<()> {
    let now = Utc::now().to_rfc3339();
    let _: () = redis_con
        .set(update_time_key(parliament, index), now)
        .await?;
    Ok(())
}

pub async fn get_update_time_of_index(
    redis_con: &mut MultiplexedConnection,
    parliament: Parliament,
    index: &Index,
) -> redis::RedisResult<chrono::DateTime<Utc>> {
    let time: String = redis_con
        .get(update_time_key(parliament, index))
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
