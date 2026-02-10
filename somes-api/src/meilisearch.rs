use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use dataservice::combx::{
    CombinedData, Decree, GovProposal, Index, OptionalVoteResult, OptionalVoteResultFilter,
    VoteResult,
};
use futures::FutureExt;
use meilisearch_sdk::settings::{PaginationSetting, Settings};
use redis::aio::MultiplexedConnection;
use reqwest::StatusCode;
use somes_common_lib::DelegateFilter;
use tokio::time::sleep;

use crate::{
    routes::{
        all_delegates, all_updated_votes_from_legis_init_sqlx, all_votes_from_legis_init,
        get_all_decrees_sqlx, get_all_gov_props, DecreeDelegate,
    },
    server::AppState,
};

mod update_time;
pub use update_time::*;

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

pub async fn update_delegates_meilisearch_index(
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut MultiplexedConnection,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all delegates..");
    let all_delegates = all_delegates(pg_pool).await?;
    log::info!("Fetched all delegates");

    let filterable_fields = DelegateFilter::filterable_fields()
        .into_iter()
        .map(|field| field.to_string())
        .collect::<Vec<String>>();

    // client.delete_index("decrees").await?;

    log::info!("Uploading {} delegates to meilisearch", all_delegates.len());
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
        .with_sortable_attributes(["name", "birthdate", "decree.mandates.start_date"])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });

    let index = Index::Delegates.as_str();
    client.index(index).set_settings(&settings).await?;

    // client.index("").delete_all_documents().await?;

    client
        .index(index)
        .add_documents_in_batches(
            &all_delegates,
            Some(3000),
            Some(OptionalVoteResult::PRIMARY_KEY),
        )
        .await?;
    update_time::update_update_time_of_index(redis_con, &Index::Delegates).await?;

    log::info!("Uploaded delegates");
    Ok(())
}

pub async fn update_decrees_meilisearch_index(
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut MultiplexedConnection,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all decrees..");
    let all_decrees = get_all_decrees_sqlx(pg_pool, redis_con.clone()).await?;
    log::info!("Fetched all decrees");

    // client.delete_index("decrees").await?;

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
        .with_filterable_attributes(["decree", "delegate"])
        .with_sortable_attributes(["decree.publication_date"])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });

    let index = Index::Decrees.as_str();
    client.index(index).set_settings(&settings).await?;

    // client.index("decrees").delete_all_documents().await?;

    client
        .index(index)
        .add_documents_in_batches(&all_decrees, Some(3000), Some(Decree::PRIMARY_KEY))
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
        .add_documents_in_batches(&all_gov_props, Some(3000), Some(GovProposal::PRIMARY_KEY))
        .await?;
    update_time::update_update_time_of_index(redis_con, &Index::GovProposals).await?;

    log::info!("Uploaded gov proposals");
    Ok(())
}

pub async fn party_of_delegate_at_time(
    date: chrono::NaiveDate,
    delegate_id: i32,
    pool: &sqlx::PgPool,
) -> sqlx::Result<Option<String>> {
    sqlx::query_scalar!("
        SELECT party
        FROM mandates m
        where is_nr and delegate_id = $2 and start_date <= $1::date AND COALESCE(end_date, $1::date) >= $1::date
        LIMIT 1", date, delegate_id)
    .fetch_one(pool).await
}

pub async fn party_of_delegates_at_time(
    date: chrono::NaiveDate,
    delegate_ids: &[i32],
    pool: &sqlx::PgPool,
) -> Vec<String> {
    let start = tokio::time::Instant::now();
    let data = futures::future::join_all(delegate_ids.iter().map(|delegate_id| async {
        party_of_delegate_at_time(date, *delegate_id, pool)
            .await
            .ok()
            .flatten()
    }))
    .map(|parties| parties.into_iter().flatten().collect::<Vec<_>>())
    .await;

    log::info!("party selection duration: {:?}", start.elapsed());

    data
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
    let mut all_vote_results = vote_result_cb(redis_con.clone(), pg_pool).await?;

    for vote_result in &mut all_vote_results {
        if let Some(meilisearch_helper) = vote_result.meilisearch_helper.as_mut() {
            meilisearch_helper.votes = vote_result
                .votes
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|vote| format!("{}{:?}", vote.party, vote.infavor))
                .collect();
        }
    }

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
    client: &redis::Client,
    dataservice_sqlx_pool: &sqlx::Pool<sqlx::Postgres>,
    meilisearch_client: &meilisearch_sdk::client::Client,
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

    let meilisearch_client_gp = meilisearch_client.clone();
    let client_vr = client.clone();

    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_decrees_meilisearch_index(
                &pg_pool,
                &mut client_vr.get_multiplexed_async_connection().await.unwrap(),
                &meilisearch_client_gp,
            )
            .await
            {
                log::error!("Could not update decree meilisearch index: {e:?}");
            }
            log::info!("decree meilsearch sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    });

    let pg_pool = dataservice_sqlx_pool.clone();
    let meilisearch_client_gp = meilisearch_client.clone();
    let client_vr = client.clone();

    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_delegates_meilisearch_index(
                &pg_pool,
                &mut client_vr.get_multiplexed_async_connection().await.unwrap(),
                &meilisearch_client_gp,
            )
            .await
            {
                log::error!("Could not update delegate meilisearch index: {e:?}");
            }
            log::info!("delegate meilsearch sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    });
}
