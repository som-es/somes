use std::fmt::Display;

use axum::{
    extract::{Path, Query},
    Json,
};
use dataservice::combx::VoteResult;
use meilisearch_sdk::search::SearchResults;
use somes_common_lib::{LegisInitFilter, Page, PartyVote, VoteResultById};

use crate::{
    meilisearch::MeilisearchClient, PgPoolConnection, RedisConnection, LEGIS_INITS_PER_PAGE,
};

pub use error::*;
mod db;
mod error;
pub mod filtering;
pub use db::*;
mod bookmark;
mod construct_vote_result;
pub use bookmark::*;

// #[utoipa::path(
//     post,
//     path = "/legis_inits",
//     params(
//         DateRange
//     ),
//     responses(
//         (status = 200, description = "Returned legislative initiatives successfully.", body = [Vec<DbLegislativeInitiativeQuery>]),
//         (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
//         (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
//     )
// )]
// pub async fn legis_inits(
//     DataserviceDbConnection(postgres_con): DataserviceDbConnection,
//     Json(filter): Json<DateRange>,
// ) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
//     postgres_con
//         .interact(|con| {
//             get_legislative_initiatives(con, filter)
//                 .map(Json)
//                 .map_err(|_| LegisInitErrorResponse::LegisInit)
//         })
//         .await
//         .map_err(|_| LegisInitErrorResponse::LegisInit)?
// }

// #[utoipa::path(
//     post,
//     path = "/latest_legis_inits",
//     params(
//         DateRange
//     ),
//     responses(
//         (status = 200, description = "Returned legislative initiatives successfully.", body = [Vec<DbLegislativeInitiativeQuery>]),
//         (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
//         (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
//     )
// )]
// pub async fn latest_legis_inits(
//     DataserviceDbConnection(postgres_con): DataserviceDbConnection,
// ) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
//     postgres_con
//         .interact(|con| {
//             get_latest_legislative_initiatives(con)
//                 .map(Json)
//                 .map_err(|_| LegisInitErrorResponse::LatestLegisInit)
//         })
//         .await
//         .map_err(|_| LegisInitErrorResponse::LatestLegisInit)?
// }

#[utoipa::path(
    post,
    path = "/latest_vote_results", 
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<VoteResult>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn latest_vote_results(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
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
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<VoteResult>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
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

pub async fn unfinished_vote_results_per_page(
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
        false,
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

pub async fn unfinished_vote_result_by_search(
    RedisConnection(redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(legis_init_filter): Json<Option<LegisInitFilter>>,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    meilisearch_for_vote_results(
        false,
        meilisearch_client,
        search_query,
        page,
        legis_init_filter,
    )
    .await
}

#[utoipa::path(
    post,
    path = "/vote_result_by_id", 
    params
    (
        Page
    ),
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<VoteResult>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn vote_result_by_id(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(vote_result_id): Query<VoteResultById>,
) -> Result<Json<VoteResult>, LegisInitErrorResponse> {
    get_vote_result_by_id(redis_con, &pg, vote_result_id.id)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_result_by_path(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, ityp, inr)): Path<(String, String, i32)>,
) -> Result<Json<VoteResult>, LegisInitErrorResponse> {
    get_vote_result_by_path(redis_con, &pg, &gp, &ityp, inr)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_result_by_search(
    RedisConnection(redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(legis_init_filter): Json<Option<LegisInitFilter>>,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    meilisearch_for_vote_results(
        true,
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

#[inline]
fn create_party_vote_filter<'a>(
    party_field: &str,
    infavor_field: &str,
    filter_values: impl Iterator<Item = &'a PartyVote>,
) -> String {
    filter_values
        .into_iter()
        .map(|filter_value| {
            format!(
                r#"["{party_field} = {}", "{infavor_field} = {}"]"#,
                filter_value.party, filter_value.infavor
            )
        })
        .collect::<Vec<_>>()
        .join(" AND ")
}

async fn meilisearch_for_vote_results(
    is_finished: bool,
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    legis_init_filter: Option<LegisInitFilter>,
) -> Result<Json<VoteResultsWithMaxPage>, LegisInitErrorResponse> {
    let mut meilisearch_filter = String::new();
    if let Some(filter) = legis_init_filter.as_ref() {
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

        meilisearch_filter = filter_conditions.join(" AND ")
    }
    log::info!("meilisearch filter: {meilisearch_filter}");

    let results: SearchResults<VoteResult> = meilisearch_client
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

    log::info!(
        "results: {:?}",
        vote_results
            .iter()
            .map(|x| x.legislative_initiative.created_at)
            .collect::<Vec<_>>()
    );

    Ok(Json(VoteResultsWithMaxPage {
        vote_results,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
    }))
}
