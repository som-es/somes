use axum::{Router, routing::get, Json, extract::Query};
use somes_common_lib::{DelegateById};
use crate::AppState;
use crate::{PgPoolConnection, routes::DelegateError};

pub fn create_political_analysis_router() -> Router<AppState> {
    Router::new()
        .route("/political_position", get(delegate_political_position))
        .route("/political_questions", get(delegate_political_questions))
        .route("/left_right_topic_score", get(left_right_topic_score_handler))
}

pub async fn delegate_political_position(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<somes_common_lib::PoliticalPosition>, DelegateError> {
    use crate::routes::delegates::delegate_political_position::extract_political_position;
    extract_political_position(delegate_by_id.delegate_id, &pg)
        .await?
        .ok_or(DelegateError::NotFound)
        .map(Json)
}

pub async fn delegate_political_questions(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<somes_common_lib::DelegateQA>>, DelegateError> {
    use crate::routes::delegates::delegate_political_position::extract_political_position_questions;
    extract_political_position_questions(delegate_by_id.delegate_id, &pg)
        .await
        .map(Json)
        .map_err(|_| DelegateError::Internal)
}

pub async fn left_right_topic_score_handler(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<somes_common_lib::StanceTopicScore>>, DelegateError> {
    use crate::routes::delegates::left_right_topic_score::extract_left_right_topic_score_by_delegate;
    extract_left_right_topic_score_by_delegate(&pg, delegate_by_id.delegate_id)
        .await
        .map(Json)
        .map_err(|_| DelegateError::Internal)
}
