use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use dataservice::combx::OptionalVoteResult;
use somes_common_lib::{LegisInitFilter, Page, ID, LATEST, LIVE, SEARCH};

use crate::{server::AppState, PgPoolConnection, RedisConnection, LEGIS_INITS_PER_PAGE};

pub use error::*;
mod db;
mod error;
pub mod filter;
mod routes;
pub use db::*;
pub use routes::*;
mod construct_vote_result;

pub fn create_vote_results_router() -> Router<AppState> {
    Router::new()
        .route(SEARCH, post(vote_results_by_search_route))
        .route(LIVE, post(vote_results_per_page_route))
        .route(LATEST, get(latest_vote_results_route))
        .route("/{gp}/{ityp}/{inr}", get(vote_result_by_path_route))
        .route(ID, get(vote_result_by_id_route))
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
pub async fn vote_results_per_page_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(legis_init_filter): Json<Option<LegisInitFilter>>,
) -> Result<Json<VoteResultsWithMaxPage>, FilterError> {
    if page.page < 0 {
        return Err(FilterError::InvalidPage(page.page as u32));
    }

    let updated_at = crate::meilisearch::get_update_time_of_index(
        &mut redis_con,
        &crate::meilisearch::Index::VoteResults,
    )
    .await
    .ok()
    .map(|date| date.naive_local());

    Ok(vote_results_per_page_sqlx(
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
        updated_at,
    })
    .map(Json)?)
}
