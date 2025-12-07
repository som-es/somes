mod error;
pub use error::*;

mod routes;
pub use routes::*;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use somes_common_lib::{BOOKMARK, RENEW_TOKEN, SEND_MAIL_INFO, TOPIC_SELECTION};
use sqlx::query_as;

use crate::{
    jwt::{renew_token_route, Claims},
    model::User,
    server::AppState,
    GenericErrorResponse, PgPoolConnection,
};

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/delete", delete(delete_account_route))
        .route(RENEW_TOKEN, post(renew_token_route))
        .route(TOPIC_SELECTION, post(add_user_topic_route))
        .route(TOPIC_SELECTION, delete(remove_user_topic_route))
        .route(TOPIC_SELECTION, get(user_topic_selection_route))
        .route(SEND_MAIL_INFO, put(update_send_mail_info_route))
        .route(SEND_MAIL_INFO, get(get_send_mail_info_route))
        .route("/", get(user_route))
        .nest(BOOKMARK, create_bookmark_router())
}

#[utoipa::path(
    post,
    path = "/user",
    // params(
    //     Claims
    // ),
    responses(
        (status = 200, description = "Returned user successfully.", body = [Vec<User>]),
        (status = 400, description = "Invalid request", body = [UserErrorResponse]),
        (status = 500, description = "Internal server error", body = [UserErrorResponse])
    )
)]
pub async fn user_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<User>, crate::error::GenericErrorResponse> {
    query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))
}
