use axum::Json;
use dataservice::db::models::DbLegislativeInitiativeQuery;
use somes_common_lib::DateRange;

use crate::{
    dataservice::{
        get_latest_legislative_initiatives, get_latest_vote_results, get_legislative_initiatives,
        VoteResult,
    },
    DataserviceDbConnection,
};

pub use error::*;
mod error;

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
    params(
        DateRange
    ),
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<VoteResult>]), 
        (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn latest_vote_results(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
) -> Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
    postgres_con
        .interact(|con| {
            get_latest_vote_results(con)
                .map(Json)
                .map_err(|_| LegisInitErrorResponse::LatestVoteResults)
        })
        .await
        .map_err(|_| LegisInitErrorResponse::LatestVoteResults)?
}
