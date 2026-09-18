use axum::{Json, extract::Query};
use somes_common_lib::DelegateByIdAndPage;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState, PgPoolConnection,
    routes::{
        DelegateError, InterjectionsWithMaxPage, extract_interjections_made_by_delegate,
        extract_interjections_received_by_delegate,
    },
};

pub fn create_delegate_interjections_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(interjections_received_by_delegate_per_page_route))
        .routes(routes!(interjections_made_by_delegate_per_page_route))
}

#[utoipa::path(
    get,
    path = "/made",
    tag = "delegates",
    params(DelegateByIdAndPage),
    responses(
        (status = 200, description = "Interjections made by delegate per page", body = InterjectionsWithMaxPage),
    )
)]
pub async fn interjections_made_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<InterjectionsWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = std::env::var("INTERJECTIONS_PER_PAGE")
        .unwrap_or_else(|_| "20".to_string())
        .parse()
        .unwrap_or(20);
    Ok(
        extract_interjections_made_by_delegate(delegate_id, page, page_elements, &pg)
            .await
            .map(Json)?,
    )
}

#[utoipa::path(
    get,
    path = "/received",
    tag = "delegates",
    params(DelegateByIdAndPage),
    responses(
        (status = 200, description = "Interjections received by delegate per page", body = InterjectionsWithMaxPage),
    )
)]
pub async fn interjections_received_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<InterjectionsWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = std::env::var("INTERJECTIONS_PER_PAGE")
        .unwrap_or_else(|_| "20".to_string())
        .parse()
        .unwrap_or(20);
    Ok(
        extract_interjections_received_by_delegate(delegate_id, page, page_elements, &pg)
            .await
            .map(Json)?,
    )
}
