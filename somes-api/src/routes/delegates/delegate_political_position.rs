use axum::{Json, extract::Query};
use somes_common_lib::{DelegateById, DelegateQA, PoliticalPosition};
use sqlx::{PgPool, query_as};

use crate::{PgPoolConnection, routes::DelegateError};

pub struct StanceAnswerQuestion {}

pub async fn extract_political_position_questions(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<Vec<DelegateQA>> {
    query_as!(
        DelegateQA,
        "select answer, question
        from political_answers inner join political_questions pq on pq.id = question_id
        where delegate_id = $1 and model_used = 'gpt4o-mini-de-run' ",
        delegate_id
    )
    .fetch_all(pg)
    .await
}

pub async fn delegate_political_questions(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<DelegateQA>>, DelegateError> {
    Ok(
        extract_political_position_questions(delegate_by_id.delegate_id, &pg)
            .await
            .map(Json)?,
    )
}

pub async fn extract_political_position(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<Option<PoliticalPosition>> {
    query_as!(
        PoliticalPosition,
        "select
        delegate_id, is_left, is_not_left, is_liberal, is_not_liberal, neutral_count
        from political_positions
        where delegate_id = $1",
        delegate_id
    )
    .fetch_optional(pg)
    .await
}

pub async fn delegate_political_position(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<PoliticalPosition>, DelegateError> {
    extract_political_position(delegate_by_id.delegate_id, &pg)
        .await?
        .ok_or(DelegateError::NotFound)
        .map(Json)
}

// pub async fn extract_political_answers_by_delegate(delegate_id: i32, pg: &PgPool) -> sqlx::Result<Vec<>> {

// }
