use sqlx::PgPool;
use tokio::time::sleep;

use crate::{routes::initiatives_with_votes, set_json_cache_no_expire};

async fn refresh_legis_inits_with_votes(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query!("refresh materialized view legislative_initiatives_with_votes")
        .execute(pool)
        .await
        .map(|_| ())
}
pub(crate) fn refresh_views(pool: &PgPool, redis_client: &redis::Client) {
    let inner_pool = pool.clone();
    let inner_client = redis_client.clone();
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = refresh_legis_inits_with_votes(&inner_pool).await {
                log::warn!("Could not refresh legis inits with votes index: {e:?}");
            } else {
                let votes = initiatives_with_votes(&inner_pool).await;
                if let Ok(votes) = &votes {
                    let _ = set_json_cache_no_expire(
                        &mut inner_client
                            .get_multiplexed_async_connection()
                            .await
                            .unwrap(),
                        "votes_only_legis_inits",
                        votes,
                    )
                    .await;
                }
            }
            sleep(std::time::Duration::from_hours(24 * 7)).await;
        }
    });
}
