use axum::Json;
use axum::extract::Query;
use reqwest::StatusCode;
use somes_common_lib::{NotificationSettings, PlatformQuery};
use sqlx::PgPool;

use crate::{GenericError, PgPoolConnection, jwt::Claims};

pub async fn upsert_notification_settings(
    pg: &PgPool,
    user_id: i32,
    platform: &str,
    settings: &NotificationSettings,
) -> Result<(), GenericError> {
    sqlx::query!(
        "insert into user_notification_settings (
            user_id,
            platform,
            send_new_vote_results,
            send_new_vote_result_by_favo,
            send_new_delegate_activity,
            send_new_ministrial_prop,
            send_new_ministrial_prop_by_favo,
            send_new_decree,
            send_new_decree_by_favo,
            send_new_proposal,
            send_new_proposal_by_favo
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (user_id, platform) do update set
            send_new_vote_results = EXCLUDED.send_new_vote_results,
            send_new_vote_result_by_favo = EXCLUDED.send_new_vote_result_by_favo,
            send_new_delegate_activity = EXCLUDED.send_new_delegate_activity,
            send_new_ministrial_prop = EXCLUDED.send_new_ministrial_prop,
            send_new_ministrial_prop_by_favo = EXCLUDED.send_new_ministrial_prop_by_favo,
            send_new_decree = EXCLUDED.send_new_decree,
            send_new_decree_by_favo = EXCLUDED.send_new_decree_by_favo,
            send_new_proposal = EXCLUDED.send_new_proposal,
            send_new_proposal_by_favo = EXCLUDED.send_new_proposal_by_favo,
            updated_at = now()",
        user_id,
        platform,
        settings.send_new_vote_results,
        settings.send_new_vote_result_by_favo,
        settings.send_new_delegate_activity,
        settings.send_new_ministrial_prop,
        settings.send_new_ministrial_prop_by_favo,
        settings.send_new_decree,
        settings.send_new_decree_by_favo,
        settings.send_new_proposal,
        settings.send_new_proposal_by_favo,
    )
    .execute(pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(())
}

pub async fn load_notification_settings(
    pg: &PgPool,
    user_id: i32,
    platform: &str,
) -> Result<NotificationSettings, GenericError> {
    let settings = sqlx::query_as!(
        NotificationSettings,
        "select
            coalesce($1::varchar, '') as \"platform!\",
            send_new_vote_results,
            send_new_vote_result_by_favo,
            send_new_delegate_activity,
            send_new_ministrial_prop,
            send_new_ministrial_prop_by_favo,
            send_new_decree,
            send_new_decree_by_favo,
            send_new_proposal,
            send_new_proposal_by_favo
        from user_notification_settings where user_id = $2 and platform = $1",
        platform,
        user_id,
    )
    .fetch_optional(pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(settings.unwrap_or_else(|| NotificationSettings {
        platform: platform.to_string(),
        ..Default::default()
    }))
}

pub async fn get_notification_settings_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Query(query): Query<PlatformQuery>,
) -> Result<Json<NotificationSettings>, GenericError> {
    let platform = platform_from_query(&query)?;
    load_notification_settings(&pg, claims.id, &platform)
        .await
        .map(Json)
}

pub async fn update_notification_settings_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(settings): Json<NotificationSettings>,
) -> Result<Json<()>, GenericError> {
    let platform = normalize_platform(&settings.platform)
        .ok_or_else(|| GenericError::Custom((StatusCode::BAD_REQUEST, "unsupported platform")))?;

    upsert_notification_settings(&pg, claims.id, &platform, &settings).await?;

    Ok(Json(()))
}

pub fn normalize_platform(platform: &str) -> Option<String> {
    match platform.trim().to_ascii_lowercase().as_str() {
        "ios" | "iphone" | "ipad" => Some("ios".into()),
        "android" => Some("android".into()),
        "web" => Some("web".into()),
        _ => None,
    }
}

fn platform_from_query(query: &PlatformQuery) -> Result<String, GenericError> {
    match query.platform.as_deref() {
        None => Ok("web".into()),
        Some(platform) => normalize_platform(platform)
            .ok_or_else(|| GenericError::Custom((StatusCode::BAD_REQUEST, "unsupported platform"))),
    }
}
