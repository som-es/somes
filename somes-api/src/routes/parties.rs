mod error;

use crate::{dataservice::get_parties, DataserviceDbConnection};
use axum::Json;
use dataservice::db::models::DbParty;

use self::error::PartiesErrorResponse;

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
