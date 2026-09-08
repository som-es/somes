use combx::{CombinedData, Decree, Index, Parliament};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;

use super::{index_settings, swap::rebuild_index_via_swap, update_time};
use crate::routes::get_all_decrees_sqlx;

pub async fn create_or_update_decrees_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all decrees..");
    let all_decrees = get_all_decrees_sqlx(pg_pool, redis_con.clone()).await?;
    log::info!("Fetched all decrees");

    let index = Index::Decrees.uid(parliament);

    log::info!("Uploading {} decrees to meilisearch", all_decrees.len());
    let settings = index_settings(&["decree", "delegate"], &["decree.publication_date"]);

    rebuild_index_via_swap(
        client,
        &index,
        &settings,
        &all_decrees,
        Some(Decree::PRIMARY_KEY),
        Some(3000),
    )
    .await?;
    update_time::update_update_time_of_index(redis_con, parliament, &Index::Decrees).await?;

    log::info!("Uploaded decrees");
    Ok(())
}
