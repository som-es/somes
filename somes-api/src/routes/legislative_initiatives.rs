use axum::Json;
use chrono::NaiveDateTime;
use dataservice::db::models::{DbLegislativeInitiative, DbVote};
use serde::{Deserialize, Serialize};

use crate::dataservice::{
    dataservice_con, get_latest_legislative_initiatives, get_legislative_initiatives, get_latest_legislative_initiatives_and_votes, VoteResult,
};

use self::error::LegisInitErrorResponse;

mod error;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RequestFilter {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

pub async fn legis_inits(
    Json(filter): Json<RequestFilter>,
) -> Result<Json<Vec<DbLegislativeInitiative>>, LegisInitErrorResponse> {
    get_legislative_initiatives(&mut dataservice_con(), filter)
        .map(|legis_inits| Json(legis_inits))
        .map_err(|_| LegisInitErrorResponse::LegisInitResponseError)
}

pub async fn latest_legis_inits(
) -> Result<Json<Vec<DbLegislativeInitiative>>, LegisInitErrorResponse> {
    get_latest_legislative_initiatives(&mut dataservice_con())
        .map(|legis_inits| Json(legis_inits))
        .map_err(|_| LegisInitErrorResponse::LegisInitResponseError)
}

pub async fn latest_legis_inits_and_votes(
) -> Result<Json<Vec<VoteResult>>, LegisInitErrorResponse> {
    get_latest_legislative_initiatives_and_votes(&mut dataservice_con())
        .map(|legis_inits| Json(legis_inits))
        .map_err(|_| LegisInitErrorResponse::LegisInitResponseError)
}
