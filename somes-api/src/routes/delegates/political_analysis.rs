use crate::AppState;
use crate::{PgPoolConnection, routes::DelegateError};
use axum::{Json, Router, extract::Query, routing::get};
use somes_common_lib::{DelegateById, PoliticalPosition};

pub fn create_political_analysis_router() -> Router<AppState> {
    Router::new().route("/political_position", get(political_position))
}

pub async fn political_position(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Option<PoliticalPosition>>, DelegateError> {
    use crate::routes::delegates::left_right_topic_score::extract_political_position_by_delegate;
    extract_political_position_by_delegate(&pg, delegate_by_id.delegate_id)
        .await
        .map(Json)
        .map_err(|_| DelegateError::Internal)
}
