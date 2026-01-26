mod error;
use std::collections::HashMap;

use crate::PgPoolConnection;
use axum::{extract::Query, Json};
use common_scrapes::Party;
pub use error::*;
use somes_common_lib::LegisPeriodGp;

#[utoipa::path(
    get,
    path = "/parties", 
    responses(
        // (status = 200, description = "Returned parties successfully.", body = [Vec<Party>]), 
        // (status = 400, description = "Invalid request", body = [PartiesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [PartiesErrorResponse])
    )
)]
pub async fn parties_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    Ok(dataservice::combx::with_data::all_parties(&pg)
        .await
        .map(Json)?)
}

pub async fn parties_per_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<HashMap<String, Vec<Party>>>, PartiesErrorResponse> {
    Ok(dataservice::combx::with_data::all_parties_per_gp(&pg)
        .await
        .map(Json)?)
}

pub async fn parties_at_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(legis_period): Query<LegisPeriodGp>,
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    Ok(
        dataservice::combx::with_data::all_parties_at_gp(&pg, &legis_period.gp)
            .await
            .map(Json)?,
    )
}
