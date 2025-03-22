use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use meilisearch_sdk::settings::Settings;
use redis::aio::MultiplexedConnection;
use reqwest::StatusCode;

use crate::{
    routes::{get_all_gov_props, get_all_votes_from_legis_init},
    server::AppState,
};

#[derive(FromRef)]
pub struct MeilisearchClient(pub meilisearch_sdk::client::Client);

#[async_trait]
impl FromRequestParts<AppState> for MeilisearchClient {
    type Rejection = (StatusCode, String);

    #[inline]
    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.meilisearch_client.clone()))
    }
}

pub async fn update_gov_props_meilisearch_index(
    redis_con: &mut MultiplexedConnection,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all gov proposals..");
    let all_gov_props = get_all_gov_props(redis_con.clone(), pg_pool).await?;
    log::info!("Fetched all gov proposals");

    // client.delete_index("vote_results").await?;

    log::info!(
        "Uploading {} gov proposals to meilisearch",
        all_gov_props.len()
    );
    let settings = Settings::new().with_filterable_attributes([
        "gov_proposal.ministrial_proposal.gp",
        "gov_proposal.ministrial_proposal.has_vote_result",
    ]);

    client.index("gov_props").set_settings(&settings).await?;

    client
        .index("gov_props")
        .add_documents(&all_gov_props, Some("gov_proposal.ministrial_proposal.id"))
        .await?;

    log::info!("Uploaded gov proposals");
    Ok(())
}

pub async fn update_vote_result_meilisearch_index(
    redis_con: &mut MultiplexedConnection,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all vote results..");
    let all_vote_results = get_all_votes_from_legis_init(redis_con.clone(), pg_pool).await?;
    log::info!("Fetched all vote results");

    // client.delete_index("vote_results").await?;

    log::info!(
        "Uploading {} vote results to meilisearch",
        all_vote_results.len()
    );
    let settings = Settings::new().with_filterable_attributes([
        "legislative_initiative.accepted",
        "legislative_initiative.requires_simple_majority",
        "legislative_initiative.gp",
        "legislative_initiative.voted_by_name",
    ]);

    client.index("vote_results").set_settings(&settings).await?;

    client
        .index("vote_results")
        .add_documents(&all_vote_results, Some("id"))
        .await?;

    log::info!("Uploaded vote results");
    Ok(())
}
