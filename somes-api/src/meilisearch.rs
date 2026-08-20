use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use combx::{
    CombinedData, Decree, DelegateFilter, GovProposal, Index, OptionalVoteResult,
    OptionalVoteResultFilter, Parliament,
};
use futures::FutureExt;
use meilisearch_sdk::{
    client::Client,
    errors::{Error, ErrorCode, MeilisearchError},
    settings::{PaginationSetting, Settings},
};
use redis::aio::ConnectionManager;
use reqwest::StatusCode;
use tokio::time::sleep;

use crate::{
    AppState, IS_PROD,
    routes::{all_delegates, all_votes_from_legis_init, get_all_decrees_sqlx, get_all_gov_props},
};

pub mod update_time;
pub use update_time::*;

const MEILISEARCH_TASK_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(600);
const MEILISEARCH_TASK_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(200);

async fn index_exists(client: &Client, uid: &str) -> Result<bool, Error> {
    match client.get_index(uid).await {
        Ok(_) => Ok(true),
        Err(Error::Meilisearch(MeilisearchError {
            error_code: ErrorCode::IndexNotFound,
            ..
        })) => Ok(false),
        Err(e) => Err(e), // network error, auth error, etc.
    }
}

async fn rebuild_index_via_swap<T: serde::Serialize + Send + Sync>(
    client: &meilisearch_sdk::client::Client,
    index: &str,
    settings: &Settings,
    documents: &[T],
    primary_key: Option<&str>,
    batch_size: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let swap_index = format!("{index}_swap");

    if let Ok(exists) = index_exists(client, index).await {
        if !exists {
            let task = client.create_index(index, None).await?;
            task.wait_for_completion(client, None, None).await?;
        }
    }

    client
        .index(&swap_index)
        .set_settings(settings)
        .await?
        .wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;

    let upload_tasks = client
        .index(&swap_index)
        .add_documents_in_batches(documents, batch_size, primary_key)
        .await?;

    for task in upload_tasks {
        task.wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;
    }

    client
        .swap_indexes(&[meilisearch_sdk::client::SwapIndexes {
            indexes: (index.to_string(), swap_index.clone()),
            rename: None,
        }])
        .await?
        .wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;

    client.delete_index(&swap_index).await?;

    Ok(())
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

pub async fn update_delegates_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &meilisearch_sdk::client::Client,
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

pub async fn create_or_update_decrees_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all decrees..");
    let all_decrees = get_all_decrees_sqlx(pg_pool, redis_con.clone()).await?;
    log::info!("Fetched all decrees");

    let index = Index::Decrees.uid(parliament);

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

pub async fn create_or_update_gov_props_meilisearch_index(
    parliament: Parliament,
    redis_con: &mut ConnectionManager,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all gov proposals..");
    let all_gov_props = get_all_gov_props(redis_con.clone(), pg_pool).await?;
    log::info!("Fetched all gov proposals");

    let index = Index::GovProposals.uid(parliament);

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
    parliament: Parliament,
    redis_con: &mut ConnectionManager,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
    vote_result_cb: impl AsyncFn(
        ConnectionManager,
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
        .with_sortable_attributes([
            "legislative_initiative.nr_plenary_activity_date",
            "legislative_initiative.raw_data_created_at",
            "legislative_initiative.vote_date",
        ])
        .with_pagination(PaginationSetting {
            max_total_hits: 100000000,
        });

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

fn spawn_parliament_index_refreshers(
    parliament: Parliament,
    client: &ConnectionManager,
    dataservice_sqlx_pool: &sqlx::Pool<sqlx::Postgres>,
    meilisearch_client: &meilisearch_sdk::client::Client,
    prod_wait_handles: &mut Vec<tokio::task::JoinHandle<()>>,
) {
    let pg_pool_vr = dataservice_sqlx_pool.clone();
    let mut client_vr = client.clone();
    let meilisearch_client_vr = meilisearch_client.clone();

    prod_wait_handles.push(tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_vote_result_meilisearch_index(
                parliament,
                &mut client_vr,
                &pg_pool_vr,
                &meilisearch_client_vr,
                all_votes_from_legis_init,
            )
            .await
            {
                log::warn!(
                    "Could not update meilisearch index {} ({parliament}): {e:?}",
                    Index::VoteResults.uid(parliament)
                );
            }
            if *IS_PROD {
                break;
            }
            sleep(std::time::Duration::from_secs(1900)).await;
        }
    }));

    let pg_pool = dataservice_sqlx_pool.clone();
    let meilisearch_client_gp = meilisearch_client.clone();
    let mut client_vr = client.clone();

    prod_wait_handles.push(tokio::task::spawn(async move {
        loop {
            if let Err(e) = create_or_update_gov_props_meilisearch_index(
                parliament,
                &mut client_vr,
                &pg_pool,
                &meilisearch_client_gp,
            )
            .await
            {
                log::warn!(
                    "Could not update meilisearch index {} ({parliament}): {e:?}",
                    Index::GovProposals.uid(parliament)
                );
            }
            if *IS_PROD {
                break;
            }
            log::info!("gov prop sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    }));

    let pg_pool = dataservice_sqlx_pool.clone();

    let meilisearch_client_gp = meilisearch_client.clone();
    let mut client_vr = client.clone();

    tokio::task::spawn(async move {
        loop {
            if let Err(e) = create_or_update_decrees_meilisearch_index(
                parliament,
                &pg_pool,
                &mut client_vr,
                &meilisearch_client_gp,
            )
            .await
            {
                log::error!(
                    "Could not update meilisearch index {} ({parliament}): {e:?}",
                    Index::Decrees.uid(parliament)
                );
            }
            log::info!("decree meilsearch sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    });

    let pg_pool = dataservice_sqlx_pool.clone();
    let meilisearch_client_gp = meilisearch_client.clone();
    let mut client_vr = client.clone();

    prod_wait_handles.push(tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_delegates_meilisearch_index(
                parliament,
                &pg_pool,
                &mut client_vr,
                &meilisearch_client_gp,
            )
            .await
            {
                log::error!(
                    "Could not update meilisearch index {} ({parliament}): {e:?}",
                    Index::Delegates.uid(parliament)
                );
            }
            if *IS_PROD {
                break;
            }
            log::info!("delegate meilsearch sleep 1000s");
            sleep(std::time::Duration::from_secs(1000)).await;
        }
    }));
}

pub async fn update_meilisearch_indices(app_state: &AppState) {
    let mut prod_wait_handles = vec![];

    spawn_parliament_index_refreshers(
        Parliament::At,
        &app_state.redis.connection,
        &app_state.dataservice_sqlx_pool,
        &app_state.meilisearch_client,
        &mut prod_wait_handles,
    );

    spawn_parliament_index_refreshers(
        Parliament::Eu,
        &app_state.eu_redis.connection,
        &app_state.eu_dataservice_sqlx_pool,
        &app_state.meilisearch_client,
        &mut prod_wait_handles,
    );

    if *IS_PROD {
        for handle in prod_wait_handles {
            if let Err(e) = handle.await {
                log::error!("Could not force update for cache: {e:}")
            }
        }
    }
}
