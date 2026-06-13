use sqlx::PgPool;

use combx::models::*;

pub async fn extract_parliamentary_questions(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<Vec<ParliamentQuestionResponse>> {
    sqlx::query_as!(
        ParliamentQuestionResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentQuestion").data).issuer_ids)
        "#,
        delegate_id as i64
    )
    .fetch_all(pg)
    .await
}
pub async fn extract_parliamentary_answers(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<Vec<ParliamentQuestionResponse>> {
    sqlx::query_as!(
        ParliamentQuestionResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentQuestion").data).receiver_ids)
        "#,
        delegate_id as i64
    )
    .fetch_all(pg)
    .await
}
