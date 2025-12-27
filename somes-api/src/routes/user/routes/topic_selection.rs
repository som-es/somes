use axum::Json;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::query_as;

use crate::{jwt::Claims, routes::UniqueTopic, GenericError, PgPoolConnection};

pub async fn add_user_topic_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(topic): Json<UniqueTopic>,
) -> Result<Json<()>, GenericError> {
    let exists = sqlx::query_scalar!(
        "select exists(select 1 from unique_eurovoc_topics where id = $1)",
        topic.id
    )
    .fetch_one(&pg)
    .await
    .map_err(|_| GenericError::SqlFailure(None))?;

    if !exists.unwrap_or(false) {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "topic_id does not exist",
        )));
    }

    query_as!(
        UniqueTopic,
        "insert into user_topics(user_id, topic_id) values ($1, $2) on conflict do nothing",
        claims.id,
        topic.id
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|_| GenericError::SqlFailure(None))
}

pub async fn user_topic_selection_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<UniqueTopic>>, GenericError> {
    query_as!(
        UniqueTopic,
        "select topic_id as id, topic_name as topic from user_topics inner join unique_eurovoc_topics as ut on ut.id = topic_id where user_id = $1",
        claims.id,
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| GenericError::SqlFailure(None))
}

pub async fn remove_user_topic_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(topic): Json<UniqueTopic>,
) -> Result<Json<()>, GenericError> {
    query_as!(
        UniqueTopic,
        "delete from user_topics where user_id = $1 and topic_id = $2",
        claims.id,
        topic.id
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|_| GenericError::SqlFailure(None))
}
