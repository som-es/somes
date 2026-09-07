use axum::Json;
use reqwest::StatusCode;
use somes_common_lib::{PushTokenDeletion, PushTokenInfo, PushTokenRegistration};

use crate::{GenericError, PgPoolConnection, jwt::Claims};

use super::normalize_platform;

pub async fn register_push_token_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(registration): Json<PushTokenRegistration>,
) -> Result<Json<()>, GenericError> {
    let platform = normalize_platform(&registration.platform)
        .ok_or_else(|| GenericError::Custom((StatusCode::BAD_REQUEST, "unsupported platform")))?;
    let push_token = registration.push_token.trim();

    if !is_valid_push_token(push_token) {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "invalid push token",
        )));
    }

    let settings = sqlx::query!(
        "insert into user_notification_settings (user_id, platform) values ($1, $2)
        on conflict (user_id, platform) do update set updated_at = now()
        returning id",
        claims.id,
        platform,
    )
    .fetch_one(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    sqlx::query!(
        "insert into expo_push_tokens (user_id, notification_settings_id, push_token, platform)
        values ($1, $2, $3, $4)
        on conflict (push_token) do update set
            user_id = EXCLUDED.user_id,
            notification_settings_id = EXCLUDED.notification_settings_id,
            platform = EXCLUDED.platform,
            enabled = true,
            updated_at = now()",
        claims.id,
        settings.id,
        push_token,
        platform,
    )
    .execute(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(Json(()))
}

pub async fn remove_push_token_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(deletion): Json<PushTokenDeletion>,
) -> Result<Json<()>, GenericError> {
    let push_token = deletion.push_token.trim();

    if push_token.is_empty() {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "invalid push token",
        )));
    }

    sqlx::query!(
        "delete from expo_push_tokens where push_token = $1 and user_id = $2",
        push_token,
        claims.id,
    )
    .execute(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(Json(()))
}

pub async fn user_push_tokens_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<PushTokenInfo>>, GenericError> {
    let tokens = sqlx::query_as!(
        PushTokenInfo,
        "select push_token, platform, enabled from expo_push_tokens where user_id = $1 and enabled order by id",
        claims.id,
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(Json(tokens))
}

pub fn is_valid_push_token(push_token: &str) -> bool {
    push_token.len() <= 255
        && push_token.ends_with(']')
        && (push_token.starts_with("ExponentPushToken[")
            || push_token.starts_with("ExpoPushToken["))
}
