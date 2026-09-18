use axum::{Json, extract::Query};
use combx::Index;
use somes_common_lib::{AddonVoteResultFilter, Page};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{AppState, LEGIS_INITS_PER_PAGE, ParliamentCtx, PgPoolConnection, RedisConnection};

pub use error::*;
mod db;
mod error;
pub mod filter;
mod routes;
pub use db::*;
pub use routes::*;
mod construct_vote_result;
pub use construct_vote_result::*;

pub fn create_vote_results_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(vote_results_by_search_route))
        .routes(routes!(vote_results_per_page_route))
        .routes(routes!(latest_vote_results_route))
        .routes(routes!(vote_result_by_path_route))
        .routes(routes!(vote_result_by_id_route))
}

#[utoipa::path(
    post,
    path = "/live",
    tag = "vote_results",
    params(Page),
    request_body(content = AddonVoteResultFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Vote results per page", body = VoteResultsWithMaxPage),
    )
)]
pub async fn vote_results_per_page_route(
    ParliamentCtx(parliament): ParliamentCtx,
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(legis_init_filter): Json<Option<AddonVoteResultFilter>>,
) -> Result<Json<VoteResultsWithMaxPage>, FilterError> {
    if page.page < 0 {
        return Err(FilterError::InvalidPage(page.page as u32));
    }

    let updated_at = crate::meilisearch::get_update_time_of_index(
        &mut redis_con,
        parliament,
        &Index::VoteResults,
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
