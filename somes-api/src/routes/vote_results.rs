use std::fmt::Display;

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use dataservice::combx::{vote_results, OptionalVoteResult};
use meilisearch_sdk::search::SearchResults;
use somes_common_lib::{LegisInitFilter, Page, VoteResultById, ID, LATEST, LIVE, SEARCH};

use crate::{
    meilisearch::MeilisearchClient, server::AppState, PgPoolConnection, RedisConnection,
    LEGIS_INITS_PER_PAGE,
};

pub use error::*;
mod db;
mod error;
pub mod filter;
pub use db::*;
mod construct_vote_result;

pub fn create_vote_results_router() -> Router<AppState> {
    Router::new()
        .route(SEARCH, post(vote_results_by_search))
        .route(LIVE, post(vote_results_per_page))
        .route(LATEST, post(latest_vote_results))
        .route("/{gp}/{ityp}/{inr}", get(vote_result_by_path))
        .route(ID, get(vote_result_by_id))
}

#[utoipa::path(
    post,
    path = "/latest_vote_results",
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<OptionalVoteResult>]),
        // (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        // (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn latest_vote_results(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<OptionalVoteResult>>, LegisInitErrorResponse> {
    get_latest_vote_results_sqlx(redis_con, &pg)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::LatestVoteResults)
}

#[utoipa::path(
    post,
    path = "/vote_results_per_page",
    params
    (
        Page
    ),
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<OptionalVoteResult>]),
        // (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        // (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn vote_results_per_page(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(legis_init_filter): Json<Option<LegisInitFilter>>,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    if page.page < 0 {
        return Err(LegisInitErrorResponse::InvalidPage);
    }
    // if page.page > page_count {
    //     return Err(LegisInitErrorResponse::InvalidPage);
    // }

    get_latest_vote_results_sqlx_per_page(
        redis_con,
        &pg,
        page.page,
        LEGIS_INITS_PER_PAGE.parse().unwrap_or(16),
        legis_init_filter.as_ref(),
        true,
    )
    .await
    .map(|(vote_results, entry_count)| VoteResultsWithMaxPage {
        vote_results,
        entry_count,
        max_page: (entry_count as f64 / LEGIS_INITS_PER_PAGE.parse().unwrap_or(16.)).ceil() as i64,
    })
    .map(Json)
    .map_err(|_| LegisInitErrorResponse::LatestVoteResults)
}

#[utoipa::path(
    post,
    path = "/vote_result_by_id",
    params
    (
        Page
    ),
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<OptionalVoteResult>]),
        // (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        // (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn vote_result_by_id(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(vote_result_id): Query<VoteResultById>,
) -> Result<Json<OptionalVoteResult>, LegisInitErrorResponse> {
    get_vote_result_by_id(redis_con, &pg, vote_result_id.id)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_result_by_path(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, ityp, inr)): Path<(String, String, i32)>,
) -> Result<Json<OptionalVoteResult>, LegisInitErrorResponse> {
    vote_result_by_path_sqlx(redis_con, &pg, &gp, &ityp, inr)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_results_by_search(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(legis_init_filter): Json<LegisInitFilter>,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    meilisearch_for_vote_results(
        legis_init_filter.is_finished,
        meilisearch_client,
        search_query,
        page,
        legis_init_filter,
    )
    .await
}

#[inline]
fn create_topic_filter<T: Display>(field: &str, filter_values: impl Iterator<Item = T>) -> String {
    filter_values
        .into_iter()
        .map(|filter_value| format!("{field} = {filter_value}"))
        .collect::<Vec<_>>()
        .join(" AND ")
}

async fn meilisearch_for_vote_results(
    is_finished: bool,
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    filter: LegisInitFilter,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    let mut filter_conditions = if is_finished {
        vec![r#"legislative_initiative.accepted IS NOT NULL"#.to_string()]
    } else {
        vec![r#"legislative_initiative.accepted IS NULL AND legislative_initiative.has_reference = false"#.to_string()]
    };

    if let Some(topics) = &filter.topics {
        filter_conditions.push(create_topic_filter("topics.topic", topics.iter()));
    }

    if let Some(party_votes) = &filter.party_votes {
        filter_conditions.push(create_topic_filter(
            "meilisearch_helper.votes",
            party_votes
                .iter()
                .map(|vote| format!("{}{:?}", vote.party, vote.infavor)),
        ));
        // filter_conditions.push(create_party_vote_filter(
        //     "votes.party",
        //     "votes.infavor",
        //     party_votes.iter(),
        // ));
    }

    if let Some(accepted) = &filter.accepted {
        filter_conditions.push(format!("legislative_initiative.accepted = '{}'", accepted));
    }
    if let Some(simple_majority) = filter.simple_majority {
        filter_conditions.push(format!(
            "legislative_initiative.requires_simple_majority = {}",
            simple_majority
        ));
    }
    if let Some(ref legis_period) = filter.legis_period {
        filter_conditions.push(format!("legislative_initiative.gp = '{}'", legis_period));
    }

    if let Some(is_named_vote) = filter.is_named_vote {
        filter_conditions.push(format!(
            "legislative_initiative.voted_by_name = {}",
            is_named_vote
        ));
    }

    if let Some(is_law) = filter.is_law {
        filter_conditions.push(format!("legislative_initiative.is_law = {}", is_law));
    }

    if let Some(voting) = filter.vote_type {
        filter_conditions.push(format!("legislative_initiative.voting = {voting}"));
    }

    if let Some(is_urgent) = filter.is_urgent {
        filter_conditions.push(format!("legislative_initiative.is_urgent = {is_urgent}"));
    }

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("vote results meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let results: SearchResults<OptionalVoteResult> = meilisearch_client
        .index("vote_results")
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&["legislative_initiative.created_at:desc"])
        .with_query(&search_query.search)
        .with_hits_per_page(LEGIS_INITS_PER_PAGE.parse().unwrap_or(16))
        .with_page(page.page as usize)
        .execute()
        .await
        .unwrap();

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let vote_results = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();

    // log::info!(
    //     "results: {:?}",
    //     vote_results
    //         .iter()
    //         .map(|x| x.legislative_initiative.created_at)
    //         .collect::<Vec<_>>()
    // );

    Ok(Json(VoteResultsWithMaxPage {
        vote_results,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
    }))
}
