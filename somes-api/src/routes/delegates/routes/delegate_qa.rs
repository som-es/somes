use crate::{routes::DelegatesErrorResponse, PgPoolConnection};
use axum::{extract::Path, Json};
use somes_common_lib::DelegateQA;
use sqlx::{query_as, PgPool};

pub async fn extract_delegate_qa(delegate_id: i32, pg: &PgPool) -> sqlx::Result<Vec<DelegateQA>> {
    query_as!(DelegateQA, "select
        question, answer
        from introduction_transcriptions
        inner join transcriptionsqa
        on introduction_transcriptions.id = transcriptionsqa.transcription_id where delegate_id = $1",
    delegate_id).fetch_all(pg).await
}

pub async fn delegate_qa_route(
    PgPoolConnection(pg): PgPoolConnection,
    Path(delegate_id): Path<i32>,
) -> Result<Json<Vec<DelegateQA>>, DelegatesErrorResponse> {
    extract_delegate_qa(delegate_id, &pg)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateInterestsResponseError)
}
