mod error;

pub use error::*;

use axum::Json;
use sqlx::query_as;

use crate::{jwt::Claims, model::User, GenericErrorResponse, PgPoolConnection};

#[utoipa::path(
    post,
    path = "/user", 
    params(
        Claims
    ),
    responses(
        (status = 200, description = "Returned user successfully.", body = [Vec<UserInfo>]), 
        (status = 400, description = "Invalid request", body = [UserErrorResponse]),
        (status = 500, description = "Internal server error", body = [UserErrorResponse])
    )
)]
pub async fn user(
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
