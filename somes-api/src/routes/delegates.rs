use axum::Json;
use dataservice::db::models::{DbDelegate, DbProposalQuery};

use crate::{
    dataservice::{dataservice_con, get_delegates, get_proposals},
    routes::delegates::error::DelegatesErrorResponse,
};

mod error;

pub async fn delegates() -> Result<Json<Vec<DbDelegate>>, DelegatesErrorResponse> {
    get_delegates(&mut dataservice_con())
        .map(|delegates| Json(delegates))
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn proposals() -> Result<Json<Vec<DbProposalQuery>>, DelegatesErrorResponse> {
    get_proposals(&mut dataservice_con())
        .map(|proposal| Json(proposal))
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}
