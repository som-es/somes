use axum::Json;
use chrono::NaiveDateTime;
use dataservice::db::models::DbLegislativeInitiative;
use serde::{Deserialize, Serialize};

use crate::dataservice::{dataservice_con, get_legislative_initiatives};

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
