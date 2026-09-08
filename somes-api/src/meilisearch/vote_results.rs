use combx::{Index, OptionalVoteResult, OptionalVoteResultFilter, Parliament};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;

use super::{index_settings, swap::rebuild_index_via_swap, update_time};

pub async fn update_vote_result_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &Client,
    vote_result_cb: impl AsyncFn(
        ConnectionManager,
        &sqlx::PgPool,
    ) -> sqlx::Result<Vec<OptionalVoteResult>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filterable_fields = OptionalVoteResultFilter::filterable_fields()
        .into_iter()
        .map(|field| field.to_string())
        .filter(|field| field != "speeches" && field != "named_votes")
        .collect::<Vec<String>>();

    let settings = index_settings(
        &filterable_fields
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
        &[
            "legislative_initiative.nr_plenary_activity_date",
            "legislative_initiative.raw_data_created_at",
            "legislative_initiative.vote_date",
        ],
    );

    log::info!("Fetching all vote results..");
    let mut all_vote_results = vote_result_cb(redis_con.clone(), pg_pool).await?;

    let index = Index::VoteResults.uid(parliament);

    for vote_result in &mut all_vote_results {
        if let Some(meilisearch_helper) = vote_result.meilisearch_helper.as_mut() {
            meilisearch_helper.votes = vote_result
                .votes
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|vote| format!("{}{:?}", vote.party, vote.infavor_count > 0))
                .collect();
        }
        vote_result.speeches = None;
        if let Some(named_votes) = vote_result.named_votes.as_mut() {
            named_votes.named_votes = None;
        }
    }

    log::info!("Fetched all vote results");

    log::info!(
        "Uploading {} vote results to meilisearch",
        all_vote_results.len()
    );

    rebuild_index_via_swap(
        client,
        &index,
        &settings,
        &all_vote_results,
        Some("id"),
        Some(1000),
    )
    .await?;
    update_time::update_update_time_of_index(redis_con, parliament, &Index::VoteResults).await?;

    log::info!("Uploaded vote results");
    Ok(())
}
