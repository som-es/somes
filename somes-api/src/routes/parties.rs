mod error;
use crate::{dataservice::get_parties, DataserviceDbConnection, PgPoolConnection};
use axum::{extract::Query, Json};
use common_scrapes::Party;
use dataservice::db::models::DbParty;
pub use error::*;
use somes_common_lib::{Date, LegisPeriodGp};

#[utoipa::path(
    get,
    path = "/parties", 
    responses(
        (status = 200, description = "Returned parties successfully.", body = [Vec<DbParty>]), 
        (status = 400, description = "Invalid request", body = [PartiesErrorResponse]),
        (status = 500, description = "Internal server error", body = [PartiesErrorResponse])
    )
)]
pub async fn parties(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    dataservice::with_data::all_parties(&pg)
        .await
        .map(Json)
        .map_err(|_| PartiesErrorResponse::PartiesReturn)

    // con.interact(|con| {
    //     get_parties(con)
    //         .map(Json)
    //         .map_err(|_| PartiesErrorResponse::PartiesReturn)
    // })
    // .await
    // .map_err(|_| PartiesErrorResponse::PartiesReturn)?
}

pub async fn parties_at_gp(
    PgPoolConnection(pg): PgPoolConnection,
    Query(legis_period): Query<LegisPeriodGp>
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    dataservice::with_data::all_parties_at_gp(&pg, &legis_period.gp)
        .await
        .map(Json)
        .map_err(|_| PartiesErrorResponse::PartiesReturn)
}
