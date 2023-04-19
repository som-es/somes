use axum::Json;
use dataservice::db::models::DbDelegate;

use crate::{
    dataservice::get_delegates, establish_connection,
    routes::delegates::error::DelegatesErrorResponse,
};

mod error;

pub async fn delegates() -> Result<Json<Vec<DbDelegate>>, DelegatesErrorResponse> {
    get_delegates(&mut establish_connection())
        .map(|delegates| Json(delegates))
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}
