mod error;
use std::sync::Arc;

use combx::Parliament;
pub use error::*;

mod routes;
use reqwest::StatusCode;
pub use routes::*;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use somes_common_lib::{BOOKMARK, LOGIN_ROUTE, RENEW_TOKEN, SEND_MAIL_INFO, TOPIC_SELECTION};
use sqlx::query_as;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::{
    jwt::{renew_token_route, Claims},
    model::User,
    AppState, AtPgPoolConnection, ParliamentCtx, PgPoolConnection,
};

pub fn create_user_info_router() -> Router<AppState> {
    Router::new()
        .route(TOPIC_SELECTION, post(add_user_topic_route))
        .route(TOPIC_SELECTION, delete(remove_user_topic_route))
        .route(TOPIC_SELECTION, get(user_topic_selection_route))
        .route(SEND_MAIL_INFO, put(update_send_mail_info_route))
        .route(SEND_MAIL_INFO, get(get_send_mail_info_route))
        .nest(BOOKMARK, create_bookmark_router())
}

pub fn create_user_router() -> Router<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(4)
            .finish()
            .unwrap(),
    );

    Router::new()
        .route(
            LOGIN_ROUTE,
            post(login).layer(GovernorLayer::new(governor_conf)),
        )
        .route("/delete", delete(delete_account_route))
        .route(RENEW_TOKEN, post(renew_token_route))
        .route("/change_email", post(change_mail))
        .route("/verify_email_change", post(verify_email_change))
        .route("/anonymize_email", post(anonymize_email))
        .route("/", get(user_route))
        .route("/init", get(user_init_route))
        .merge(create_user_info_router())
}

#[utoipa::path(
    post,
    path = "/user",
    // params(
    //     Claims
    // ),
    responses(
        (status = 200, description = "Returned user successfully.", body = [Vec<User>]),
        // (status = 400, description = "Invalid request", body = [UserError]),
        // (status = 500, description = "Internal server error", body = [UserError])
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
