use combx::{CombinedData, DelegateFilter, Index, OptionalVoteResult, Parliament};
use meilisearch_sdk::client::Client;

use redis::aio::ConnectionManager;

use super::{index_settings, swap::rebuild_index_via_swap, update_time};
use crate::routes::all_delegates;

pub async fn update_delegates_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all delegates..");
    let all_delegates = all_delegates(pg_pool).await?;
    log::info!("Fetched all delegates");

    let filterable_fields = DelegateFilter::filterable_fields()
        .into_iter()
        .map(|field| field.to_string())
        .collect::<Vec<String>>();

    let index = Index::Delegates.uid(parliament);

    log::info!("Uploading {} delegates to meilisearch", all_delegates.len());
    let settings = index_settings(
        &filterable_fields
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
        &["name", "birthdate", "decree.mandates.start_date"],
    );

    rebuild_index_via_swap(
        client,
        &index,
        &settings,
        &all_delegates,
        Some(OptionalVoteResult::PRIMARY_KEY),
        Some(3000),
    )
    .await?;
    update_time::update_update_time_of_index(redis_con, parliament, &Index::Delegates).await?;

    log::info!("Uploaded delegates");
    Ok(())
}
