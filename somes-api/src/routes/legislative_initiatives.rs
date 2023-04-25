use axum::Json;
use chrono::NaiveDateTime;
use dataservice::db::models::DbLegislativeInitiative;

use self::error::LegisInitErrorResponse;

mod error;

pub struct RequestFilter {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

pub async fn legis_inits(
    Json(filter): Json<RequestFilter>,
) -> Result<Json<Vec<DbLegislativeInitiative>>, LegisInitErrorResponse> {
    todo!()
}
