use axum::{Json, extract::Query};
use dataservice::db::models::{DbDelegate, DbProposalQuery};
use somes_common_lib::DelegateById;

use crate::{
    dataservice::{get_delegates, get_proposals, get_delegate},
    DataserviceDbConnection,
};

pub use error::*;
mod error;

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate", 
    responses(
        (status = 200, description = "Returned delegate successfully.", body = [DbDelegate]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegate(
    DataserviceDbConnection(con): DataserviceDbConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<DbDelegate>, DelegatesErrorResponse> {
    con.interact(move |con| {
        get_delegate(con, delegate_by_id.delegate_id)
            .map(Json)
            .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
    })
    .await
    .map_err(|_| DelegatesErrorResponse::DelegateResponseError)?
}

#[utoipa::path(
    get,
    path = "/delegates", 
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<DbDelegate>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
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

#[utoipa::path(
    get,
    path = "/proposals", 
    responses(
        (status = 200, description = "Returned proposals successfully.", body = [Vec<DbProposalQuery>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
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
