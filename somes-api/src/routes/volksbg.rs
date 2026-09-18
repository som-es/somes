use axum::Json;
use combx::{DbVolksbg, OptionalVolksbgEintragungswoche};
use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{AppState, GenericError, PgPoolConnection};

pub fn create_volksbg_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(all_volksbg_weeks))
}

#[utoipa::path(
    get,
    path = "/weeks",
    tag = "volksbg",
    responses(
        (status = 200, description = "All volksbg weeks", body = [OptionalVolksbgEintragungswoche]),
    )
)]
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
