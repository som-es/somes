use dataservice::db::models::DbSpeechWithLink;
use sqlx::{query_as, PgPool};

pub async fn extract_delegate_speeches(delegate_id: i32, page: i64, page_elements: i64, pg: &PgPool) -> sqlx::Result<Vec<DbSpeechWithLink>> {
    query_as!(DbSpeechWithLink, "
        select delegate_id, legislative_initiatives_id, infavor, opinion, document_url from speeches 
        INNER JOIN speeches_html_urls ON speeches.id = speeches_html_urls.speech_id where delegate_id = $1 offset $2 limit $3
    ", 
    delegate_id, page * page_elements, page).fetch_all(pg).await
}
