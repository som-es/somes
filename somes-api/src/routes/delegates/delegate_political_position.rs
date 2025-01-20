use axum::{extract::Query, Json};
use somes_common_lib::{DelegateById, PoliticalPosition};
use sqlx::{query_as, PgPool};

use crate::PgPoolConnection;

use super::DelegatesErrorResponse;

pub async fn extract_political_position(delegate_id: i32, pg: &PgPool) -> sqlx::Result<Vec<PoliticalPosition>> {
    query_as!(PoliticalPosition, "select 
        delegate_id, is_left, is_not_left, is_liberal, is_not_liberal, neutral_count
        from political_positions 
        where delegate_id = $1", 
    delegate_id).fetch_all(pg).await
}

pub async fn delegate_political_position(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<PoliticalPosition>>, DelegatesErrorResponse> {
    extract_political_position(delegate_by_id.delegate_id, &pg)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateInterestsResponseError)
}

// pub async fn extract_political_answers_by_delegate(delegate_id: i32, pg: &PgPool) -> sqlx::Result<Vec<>> {

// }