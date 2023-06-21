use axum::Json;
use chrono::NaiveDate;
use dataservice::db::models::DbLegislativeInitiativeQuery;
use serde::{Deserialize, Serialize};

use crate::{
    dataservice::{
        dataservice_con, get_latest_legislative_initiatives, get_latest_vote_results,
        get_legislative_initiatives, VoteResult,
    },
    DataserviceDbConnection,
};

use self::error::LegisInitErrorResponse;

mod error;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RequestFilter {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub async fn legis_inits(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
    Json(filter): Json<RequestFilter>,
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
