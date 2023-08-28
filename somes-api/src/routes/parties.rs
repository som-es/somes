mod error;
use crate::{dataservice::get_parties, DataserviceDbConnection};
use axum::Json;
use dataservice::db::models::DbParty;
pub use error::*;

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
    DataserviceDbConnection(con): DataserviceDbConnection,
) -> Result<Json<Vec<DbParty>>, PartiesErrorResponse> {
    con.interact(|con| {
        get_parties(con)
            .map(Json)
            .map_err(|_| PartiesErrorResponse::PartiesReturn)
    })
    .await
    .map_err(|_| PartiesErrorResponse::PartiesReturn)?
}
