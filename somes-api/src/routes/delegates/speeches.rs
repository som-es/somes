use axum::{extract::Query, Json};
use dataservice::db::models::DbSpeechWithLink;
use somes_common_lib::DelegateByIdAndPage;
use sqlx::{query_as, PgPool};

use crate::PgPoolConnection;

use super::DelegatesErrorResponse;

pub async fn extract_delegate_speeches(delegate_id: i32, page: i64, page_elements: i64, pg: &PgPool) -> sqlx::Result<Vec<DbSpeechWithLink>> {
    query_as!(DbSpeechWithLink, "
        select delegate_id, legislative_initiatives_id, infavor, opinion, document_url from speeches 
        INNER JOIN speeches_html_urls ON speeches.id = speeches_html_urls.speech_id where delegate_id = $1 offset $2 limit $3
    ", 
    delegate_id, page * page_elements, page_elements).fetch_all(pg).await
}

pub async fn speeches_by_delegate_per_page(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<Vec<DbSpeechWithLink>>, DelegatesErrorResponse> {
    let DelegateByIdAndPage {
        delegate_id, page
    } = delegate_by_id_and_page;
    extract_delegate_speeches(delegate_id, page, 
        crate::SPEECHES_PER_PAGE.parse().unwrap_or(16),
        &pg)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateInterestsResponseError)
}