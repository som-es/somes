use axum::{Json, Router, extract::Query, routing::get};
use somes_common_lib::DelegateByIdAndPage;

use crate::{
    AppState, PgPoolConnection,
    routes::{
        DelegateError, ParliamentInquiryResponseWithMaxPage, extract_parliamentary_answers,
        extract_parliamentary_questions,
    },
};

pub fn create_delegate_pqa_router() -> Router<AppState> {
    Router::new()
        .route("/answers", get(answers_by_delegate_per_page_route))
        .route("/inquiries", get(inquiries_by_delegate_per_page_route))
}

pub async fn inquiries_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<ParliamentInquiryResponseWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = std::env::var("PQA_PER_PAGE")
        .unwrap_or_else(|_| "16".to_string())
        .parse()
        .unwrap_or(16);
    Ok(
        extract_parliamentary_questions(delegate_id, page, page_elements, &pg)
            .await
            .map(Json)?,
    )
}

pub async fn answers_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<ParliamentInquiryResponseWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = std::env::var("PQA_PER_PAGE")
        .unwrap_or_else(|_| "16".to_string())
        .parse()
        .unwrap_or(16);
    Ok(
        extract_parliamentary_answers(delegate_id, page, page_elements, &pg)
            .await
            .map(Json)?,
    )
}
