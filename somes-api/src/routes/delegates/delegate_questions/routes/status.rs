use axum::Json;
use redis::AsyncTypedCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    GenericError, RedisConnection, jwt::Claims,
    routes::delegates::delegate_questions::routes::ensure_admin,
};

const DELEGATE_QUESTIONS_STATUS_KEY: &str = "delegate_questions/status";

#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct DelegateQuestionStatus {
    pub enabled: bool,
}

pub async fn status_info_route(
    RedisConnection(mut redis): RedisConnection,
) -> crate::Result<Json<DelegateQuestionStatus>> {
    let value: Option<String> = redis
        .get(DELEGATE_QUESTIONS_STATUS_KEY)
        .await
        .map_err(GenericError::RedisFailure)?;

    let enabled = value.map(|value| value == "true").unwrap_or(false);

    Ok(Json(DelegateQuestionStatus { enabled }))
}

pub async fn toggle_status_route(
    claims: Claims,
    RedisConnection(mut redis): RedisConnection,
) -> crate::Result<Json<DelegateQuestionStatus>> {
    ensure_admin(&claims)?;
    let value: Option<String> = redis
        .get(DELEGATE_QUESTIONS_STATUS_KEY)
        .await
        .map_err(GenericError::RedisFailure)?;

    let enabled = value.map(|value| value == "true").unwrap_or(false);

    let enabled = !enabled;

    redis
        .set(DELEGATE_QUESTIONS_STATUS_KEY, enabled)
        .await
        .map_err(GenericError::RedisFailure)?;

    Ok(Json(DelegateQuestionStatus { enabled }))
}
