use axum::{extract::Query, Json};
use dataservice::db::models::DbLegislativeInitiativeQuery;
use somes_common_lib::{DateRange, Page};

use crate::{DataserviceDbConnection, PgPoolConnection, LEGIS_INITS_PER_PAGE};

pub use error::*;
mod db;
mod error;
pub use db::*;

#[utoipa::path(
    post,
    path = "/legis_inits", 
    params(
        DateRange
    ),
    responses(
        (status = 200, description = "Returned legislative initiatives successfully.", body = [Vec<DbLegislativeInitiativeQuery>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn legis_inits(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
    Json(filter): Json<DateRange>,
) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
    postgres_con
        .interact(|con| {
            get_legislative_initiatives(con, filter)
                .map(Json)
                .map_err(|_| LegisInitErrorResponse::LegisInit)
        })
        .await
        .map_err(|_| LegisInitErrorResponse::LegisInit)?
}

#[utoipa::path(
    post,
    path = "/latest_legis_inits", 
    params(
        DateRange
    ),
    responses(
        (status = 200, description = "Returned legislative initiatives successfully.", body = [Vec<DbLegislativeInitiativeQuery>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn latest_legis_inits(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
    postgres_con
        .interact(|con| {
            get_latest_legislative_initiatives(con)
                .map(Json)
                .map_err(|_| LegisInitErrorResponse::LatestLegisInit)
        })
        .await
        .map_err(|_| LegisInitErrorResponse::LatestLegisInit)?
}

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
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
    get_latest_vote_results_sqlx(&pg)
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
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
) -> sqlx::Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
    get_latest_vote_results_sqlx_per_page(
        &pg,
        page.page,
        LEGIS_INITS_PER_PAGE.parse().unwrap_or(16),
    )
    .await
    .map(Json)
    .map_err(|_| LegisInitErrorResponse::LatestVoteResults)
}
