use axum::Json;
use serde_json::json;
use sqlx::query_as;

use crate::PgPoolConnection;

use super::UniqueTopic;

pub async fn eurovoc_topics_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, Json<serde_json::Value>> {
    return query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_eurovoc_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "redis expire"})));
}

pub async fn topics_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, Json<serde_json::Value>> {
    return query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "redis expire"})));
}
