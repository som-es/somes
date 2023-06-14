use axum::Json;
use chrono::NaiveDate;
use dataservice::db::models::DbLegislativeInitiativeQuery;
use serde::{Deserialize, Serialize};

use crate::dataservice::{
    dataservice_con, get_latest_legislative_initiatives, get_latest_vote_results,
    get_legislative_initiatives, VoteResult,
};

use self::error::LegisInitErrorResponse;

mod error;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RequestFilter {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub async fn legis_inits(
    Json(filter): Json<RequestFilter>,
) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
    get_legislative_initiatives(&mut dataservice_con(), filter)
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::LegisInit)
}

pub async fn latest_legis_inits(
) -> Result<Json<Vec<DbLegislativeInitiativeQuery>>, LegisInitErrorResponse> {
    get_latest_legislative_initiatives(&mut dataservice_con())
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::LatestLegisInit)
}

pub async fn latest_vote_results() -> Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
    get_latest_vote_results(&mut dataservice_con())
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::LatestVoteResults)
}
