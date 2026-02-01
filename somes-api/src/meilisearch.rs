use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use chrono::format;
use dataservice::combx::{
    DbAiSummaryFilter, DbLegislativeInitiativeQueryFilter, MeilisearchHelper, OptionalDecreeFilter,
    OptionalVoteResult, OptionalVoteResultFilter,
};
use meilisearch_sdk::settings::{PaginationSetting, Settings};
use redis::aio::MultiplexedConnection;
use reqwest::StatusCode;
use serde_json::json;
use tokio::time::sleep;

use crate::{
    routes::{
        all_updated_votes_from_legis_init_sqlx, all_votes_from_legis_init, get_all_decrees_sqlx,
        get_all_gov_props,
    },
    server::AppState,
};

mod update_time;
pub use update_time::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index {
    VoteResults,
    GovProposals,
    Decrees,
}

impl Index {
    pub fn as_str(&self) -> &str {
        match self {
            Index::VoteResults => "vote_results",
            Index::GovProposals => "gov_props",
            Index::Decrees => "decrees",
        }
    }
}
impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(FromRef)]
pub struct MeilisearchClient(pub meilisearch_sdk::client::Client);

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

pub async fn update_decrees_meilisearch_index(
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut MultiplexedConnection,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all decrees..");
    let all_decrees = get_all_decrees_sqlx(pg_pool).await?;
    log::info!("Fetched all decrees");

    // client.delete_index("vote_results").await?;

    log::info!("Uploading {} decrees to meilisearch", all_decrees.len());
    let settings = Settings::new()
        .with_ranking_rules(vec![
            "sort".to_string(),
            "words".to_string(),
            "typo".to_string(),
            "proximity".to_string(),
            "attribute".to_string(),
            "exactness".to_string(),
        ])
        .with_filterable_attributes(OptionalDecreeFilter::filterable_fields())
        .with_sortable_attributes(["publication_date"])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });

    let index = Index::Decrees.as_str();
    client.index(index).set_settings(&settings).await?;

    // client.index("decrees").delete_all_documents().await?;

    let all_decrees = all_decrees
        .iter()
        .map(|decree| {
            let mut doc_json = serde_json::to_value(decree).unwrap();

            if let Some(publication_date) = decree.publication_date {
                let timestamp = publication_date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp();
                doc_json["publication_date"] = json!(timestamp);
            }
            if let Some(created_at) = decree.created_at {
                let timestamp = created_at.timestamp();
                doc_json["created_at"] = json!(timestamp);
            }
            if let Some(updated_at) = decree.updated_at {
                let timestamp = updated_at.timestamp();
                doc_json["updated_at"] = json!(timestamp);
            }
            doc_json
        })
        .collect::<Vec<_>>();

    client
        .index(index)
        .add_documents_in_batches(&all_decrees, Some(3000), Some("ris_id"))
        .await?;
    update_time::update_update_time_of_index(redis_con, &Index::Decrees).await?;

    log::info!("Uploaded decrees");
    Ok(())
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
    let settings = Settings::new()
        .with_ranking_rules(vec![
            "sort".to_string(),
            "words".to_string(),
            "typo".to_string(),
            "proximity".to_string(),
            "attribute".to_string(),
            "exactness".to_string(),
        ])
        .with_filterable_attributes(["gov_proposal", "delegate"])
        .with_sortable_attributes(["gov_proposal.ministrial_proposal.raw_data_created_at"])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });
    let index = Index::GovProposals.as_str();
    client.index(index).set_settings(&settings).await?;

    client.index(index).delete_all_documents().await?;

    client
        .index(index)
        .add_documents_in_batches(
            &all_gov_props,
            Some(3000),
            Some("gov_proposal.ministrial_proposal.id"),
        )
        .await?;
    update_time::update_update_time_of_index(redis_con, &Index::GovProposals).await?;

    log::info!("Uploaded gov proposals");
    Ok(())
}

pub async fn update_vote_result_meilisearch_index(
    redis_con: &mut MultiplexedConnection,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
    vote_result_cb: impl AsyncFn(
        MultiplexedConnection,
        &sqlx::PgPool,
    ) -> sqlx::Result<Vec<OptionalVoteResult>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filterable_fields = OptionalVoteResultFilter::filterable_fields()
        .into_iter()
        .map(|field| field.to_string())
        .collect::<Vec<String>>();

    let settings = Settings::new()
        .with_ranking_rules(vec![
            "sort".to_string(),
            "words".to_string(),
            "typo".to_string(),
            "proximity".to_string(),
            "attribute".to_string(),
            "exactness".to_string(),
        ])
        .with_filterable_attributes(&filterable_fields)
        .with_sortable_attributes(["legislative_initiative.nr_plenary_activity_date"])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });
    let index = Index::VoteResults.as_str();
    client.index(index).set_settings(&settings).await?;

    log::info!("Fetching all vote results..");
    let all_vote_results = vote_result_cb(redis_con.clone(), pg_pool)
        .await?
        .into_iter()
        .map(|mut vote_result| {
            vote_result.meilisearch_helper = Some(MeilisearchHelper {
                votes: vote_result
                    .votes
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|vote| format!("{}{:?}", vote.party, vote.infavor))
                    .collect(),
            });
            vote_result
        })
        .collect::<Vec<_>>();
    log::info!("Fetched all vote results");

    // this should only run when there are structural differences (type changes)
    // client.index(index).delete_all_documents().await?;
    // client.delete_index(index).await?;

    log::info!(
        "Uploading {} vote results to meilisearch",
        all_vote_results.len()
    );

    client
        .index(index)
        .add_documents_in_batches(&all_vote_results, Some(3000), Some("id"))
        .await?;
    update_time::update_update_time_of_index(redis_con, &Index::VoteResults).await?;

    log::info!("Uploaded vote results");
    Ok(())
}

pub fn update_meilisearch_indices(
    client: redis::Client,
    dataservice_sqlx_pool: sqlx::Pool<sqlx::Postgres>,
    meilisearch_client: meilisearch_sdk::client::Client,
) {
    let pg_pool_vr = dataservice_sqlx_pool.clone();
    let client_vr = client.clone();
    let meilisearch_client_vr = meilisearch_client.clone();

    // TODO: move this to dataservice
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_vote_result_meilisearch_index(
                &mut client_vr.get_multiplexed_async_connection().await.unwrap(),
                &pg_pool_vr,
                &meilisearch_client_vr,
                all_votes_from_legis_init,
            )
            .await
            {
                log::warn!("Could not update meilisearch index: {e:?}");
            }
            sleep(std::time::Duration::from_secs(1900)).await;
        }
    });

    let pg_pool_vr = dataservice_sqlx_pool.clone();
    let client_vr = client.clone();
    let meilisearch_client_vr = meilisearch_client.clone();

    // TODO: move this to dataservice
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_vote_result_meilisearch_index(
                &mut client_vr.get_multiplexed_async_connection().await.unwrap(),
                &pg_pool_vr,
                &meilisearch_client_vr,
                all_updated_votes_from_legis_init_sqlx,
            )
            .await
            {
                log::warn!("Could not update meilisearch index: {e:?}");
            }
            sleep(std::time::Duration::from_secs(30)).await;
        }
    });

    let pg_pool = dataservice_sqlx_pool.clone();
    let meilisearch_client_gp = meilisearch_client.clone();
    let client_vr = client.clone();

    // TODO: move this to dataservice
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_gov_props_meilisearch_index(
                &mut client_vr.get_multiplexed_async_connection().await.unwrap(),
                &pg_pool,
                &meilisearch_client_gp,
            )
            .await
            {
                log::warn!("Could not update meilisearch index: {e:?}");
            }
            log::info!("gov prop sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    });

    let pg_pool = dataservice_sqlx_pool.clone();

    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_decrees_meilisearch_index(
                &pg_pool,
                &mut client.get_multiplexed_async_connection().await.unwrap(),
                &meilisearch_client,
            )
            .await
            {
                log::error!("Could not update decree meilisearch index: {e:?}");
            }
            log::info!("decree meilsearch sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    });
}
