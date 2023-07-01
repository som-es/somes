use axum::Json;
use dataservice::db::models::{DbDelegate, DbProposalQuery};

use crate::{
    dataservice::{get_delegates, get_proposals},
    jwt::Claims,
    routes::delegates::error::DelegatesErrorResponse,
    DataserviceDbConnection,
};

mod error;

pub async fn delegates(
    DataserviceDbConnection(con): DataserviceDbConnection,
) -> Result<Json<Vec<DbDelegate>>, DelegatesErrorResponse> {
    con.interact(|con| {
        get_delegates(con)
            .map(Json)
            .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
    })
    .await
    .map_err(|_| DelegatesErrorResponse::DelegateResponseError)?
}

pub async fn proposals(
    DataserviceDbConnection(con): DataserviceDbConnection,
) -> Result<Json<Vec<DbProposalQuery>>, DelegatesErrorResponse> {
    con.interact(|con| {
        get_proposals(con)
            .map(Json)
            .map_err(|_| DelegatesErrorResponse::ProposalResponseError)
    })
    .await
    .map_err(|_| DelegatesErrorResponse::ProposalResponseError)?
}
