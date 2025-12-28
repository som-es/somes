use axum::Json;
use serde_json::json;
use sqlx::query_as;

use crate::{GenericError, PgPoolConnection};

use super::UniqueTopic;

pub async fn eurovoc_topics_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, GenericError> {
    query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_eurovoc_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}

pub async fn topics_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, GenericError> {
    query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}
