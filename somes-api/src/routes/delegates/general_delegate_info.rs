use axum::{extract::Query, Json};
use somes_common_lib::{DelegateById, GeneralDelegateInfo, Mandate};
use sqlx::{query_as, PgPool};

use crate::PgPoolConnection;

use super::DelegatesErrorResponse;


pub async fn extract_general_delegate_info(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<GeneralDelegateInfo> {
    let mandates = query_as!(
        Mandate,
        "
            select start_date, end_date, name from mandates where delegate_id = $1
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;
    Ok(GeneralDelegateInfo {
        mandates
    })
}

pub async fn general_delegate_info(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<GeneralDelegateInfo>, DelegatesErrorResponse> {
    extract_general_delegate_info(delegate_by_id.delegate_id, &pg)
        .await
        .map(Json)
        .map_err(|e| DelegatesErrorResponse::DbSelectFailure(Some(e)))
}
