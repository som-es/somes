use axum::{Json, Router, routing::get};
use combx::{DbVolksbg, OptionalVolksbgEintragungswoche};
use sqlx::PgPool;

use crate::{AppState, GenericError, PgPoolConnection};

pub fn create_volksbg_router() -> Router<AppState> {
    Router::new().route("/weeks", get(all_volksbg_weeks))
}

pub async fn all_volksbg_weeks(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<OptionalVolksbgEintragungswoche>>, GenericError> {
    volksbg_weeks_sqlx(&pg)
        .await
        .map_err(|e| GenericError::SqlFailure(Some(e)))
        .map(Json)
}

pub async fn volksbg_weeks_sqlx(pg: &PgPool) -> sqlx::Result<Vec<OptionalVolksbgEintragungswoche>> {
    sqlx::query_as!(
        OptionalVolksbgEintragungswoche,
        "select * from volksbg_weeks"
    )
    .fetch_all(pg)
    .await
}
