use combx::{CombinedData, GovProposal, Index, Parliament};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;

use super::{index_settings, swap::rebuild_index_via_swap, update_time};
use crate::routes::get_all_gov_props;

pub async fn create_or_update_gov_props_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all gov proposals..");
    let all_gov_props = get_all_gov_props(redis_con.clone(), pg_pool).await?;
    log::info!("Fetched all gov proposals");

    let index = Index::GovProposals.uid(parliament);

    log::info!(
        "Uploading {} gov proposals to meilisearch",
        all_gov_props.len()
    );
    let settings = index_settings(
        &["gov_proposal", "delegate"],
        &["gov_proposal.ministrial_proposal.raw_data_created_at"],
    );

    rebuild_index_via_swap(
        client,
        &index,
        &settings,
        &all_gov_props,
        Some(GovProposal::PRIMARY_KEY),
        Some(3000),
    )
    .await?;
    update_time::update_update_time_of_index(redis_con, parliament, &Index::GovProposals).await?;

    log::info!("Uploaded gov proposals");
    Ok(())
}
