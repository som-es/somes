mod error;
use std::sync::Arc;

use combx::Parliament;
pub use error::*;

mod routes;
use reqwest::StatusCode;
pub use routes::*;

use axum::Json;
use somes_common_lib::{BOOKMARK, PUSH_NOTIFICATIONS};
use sqlx::query_as;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    AppState, AtPgPoolConnection, ParliamentCtx, PgPoolConnection, jwt::Claims, model::User,
};

pub fn create_user_info_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(add_user_topic_route))
        .routes(routes!(remove_user_topic_route))
        .routes(routes!(user_topic_selection_route))
        .routes(routes!(update_send_mail_info_route))
        .routes(routes!(get_send_mail_info_route))
        .nest(BOOKMARK, create_bookmark_router())
        .nest(PUSH_NOTIFICATIONS, create_push_notification_router())
}

pub fn create_user_router() -> OpenApiRouter<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(4)
            .finish()
            .unwrap(),
    );

    OpenApiRouter::new()
        .routes(routes!(login).layer(GovernorLayer::new(governor_conf)))
        .routes(routes!(delete_account_route))
        .routes(routes!(crate::jwt::renew_token_route))
        .routes(routes!(change_mail))
        .routes(routes!(verify_email_change))
        .routes(routes!(anonymize_email))
        .routes(routes!(user_route))
        .routes(routes!(user_init_route))
        .merge(create_user_info_router())
        .merge(create_user_mcp_router())
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "User", body = User),
    )
)]
pub async fn user_route(
    claims: Claims,
    AtPgPoolConnection(pg): AtPgPoolConnection,
) -> Result<Json<User>, UserError> {
    Ok(query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await
    .map(Json)?)
}

#[utoipa::path(
    get,
    path = "/init",
    tag = "user",
    responses(
        (status = 200, description = "User init"),
    )
)]
pub async fn user_init_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    AtPgPoolConnection(at_pg): AtPgPoolConnection,
    ParliamentCtx(parliament): ParliamentCtx,
) -> Result<Json<()>, UserError> {
    if parliament == Parliament::At {
        return Ok(Json(()));
    }
    let existing = sqlx::query!("select id from somes_user where id = $1", claims.id)
        .fetch_optional(&at_pg)
        .await?;

    if existing.is_none() {
        return Err(UserError::Custom(
            StatusCode::NOT_FOUND,
            "user not found".into(),
        ));
    }

    sqlx::query!("insert into somes_user (id, is_email_hashed, email) values ($1, $2, $3) on conflict (id) do update
        set is_email_hashed = EXCLUDED.is_email_hashed, email = EXCLUDED.email
    ", claims.id, claims.is_anonymised, claims.sub).execute(&pg).await?;

    Ok(Json(()))
}
