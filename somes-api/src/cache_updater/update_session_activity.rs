use std::time::Duration;

use crate::{
    routes::{fetch_latest_session_activity_overview, SESSION_ACTIVITY_CACHE_KEY},
    set_json_cache_secs,
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(25 * 60);

pub async fn update_session_activity_cache(
    redis_client: redis::Client,
    pg: sqlx::Pool<sqlx::Postgres>,
) {
    loop {
        match fetch_latest_session_activity_overview(&pg).await {
            Ok(overview) => match redis_client.get_multiplexed_async_connection().await {
                Ok(mut con) => {
                    set_json_cache_secs(&mut con, SESSION_ACTIVITY_CACHE_KEY, &overview, 30 * 60)
                        .await;
                    log::info!("Updated session activity overview cache");
                }
                Err(e) => {
                    log::error!("Redis connection failed for session activity cache: {e:?}")
                }
            },
            Err(e) => log::error!("Failed to compute session activity overview: {e:?}"),
        }
        tokio::time::sleep(REFRESH_INTERVAL).await;
    }
}
