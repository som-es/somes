use axum::{extract::Query, routing::get, Json, Router};
use somes_common_lib::DelegateByIdAndPage;

use crate::{
    routes::{
        extract_interjections_made_by_delegate, extract_interjections_received_by_delegate,
        DelegateError, InterjectionsWithMaxPage,
    },
    server::AppState,
    PgPoolConnection,
};

pub fn create_delegate_interjections_router() -> Router<AppState> {
    Router::new()
        .route(
            "/received",
            get(interjections_received_by_delegate_per_page_route),
        )
        .route("/made", get(interjections_made_by_delegate_per_page_route))
}

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
