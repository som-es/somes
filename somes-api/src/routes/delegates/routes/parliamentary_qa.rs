use axum::{Json, extract::Query};
use somes_common_lib::DelegateByIdAndPage;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState, PgPoolConnection,
    routes::{
        DelegateError, ParliamentInquiryResponseWithMaxPage, extract_parliamentary_answers,
        extract_parliamentary_questions,
    },
};

pub fn create_delegate_pqa_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(answers_by_delegate_per_page_route))
        .routes(routes!(inquiries_by_delegate_per_page_route))
}

#[utoipa::path(
    get,
    path = "/inquiries",
    tag = "delegates",
    params(DelegateByIdAndPage),
    responses(
        (status = 200, description = "Parliamentary inquiries of the delegate"),
    )
)]
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

#[utoipa::path(
    get,
    path = "/answers",
    tag = "delegates",
    params(DelegateByIdAndPage),
    responses(
        (status = 200, description = "Parliamentary answers of the delegate"),
    )
)]
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
