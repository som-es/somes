use axum::Json;
use somes_common_lib::LegisInitFavo;
use sqlx::query_as;

use crate::{PgPoolConnection, jwt::Claims, routes::UserError};

#[utoipa::path(
    post,
    path = "/vote_result",
    tag = "user",
    request_body(content = LegisInitFavo, content_type = "application/json"),
    responses(
        (status = 200, description = "Add user vote result bookmark"),
    )
)]
pub async fn add_user_vote_result_bookmark(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<LegisInitFavo>,
) -> Result<Json<()>, UserError> {
    query_as!(
        UniqueTopic,
        "insert into favo_legis_inits(user_id, legis_init_id) values ($1, $2) on conflict do nothing",
        claims.id,
        delegate_favo.vote_result_id,
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| {
        UserError::SqlFailure(e)
    })
}

#[utoipa::path(
    get,
    path = "/vote_result",
    tag = "user",
    responses(
        (status = 200, description = "User vote result booksmarks", body = [LegisInitFavo]),
    )
)]
pub async fn user_vote_result_booksmarks(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<LegisInitFavo>>, UserError> {
    query_as!(
        LegisInitFavo,
        "select legis_init_id as vote_result_id from favo_legis_inits where user_id = $1",
        claims.id,
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| UserError::SqlFailure(e))
}

#[utoipa::path(
    delete,
    path = "/vote_result",
    tag = "user",
    request_body(content = LegisInitFavo, content_type = "application/json"),
    responses(
        (status = 200, description = "Remove user vote result bookmark"),
    )
)]
pub async fn remove_user_vote_result_bookmark(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<LegisInitFavo>,
) -> Result<Json<()>, UserError> {
    query_as!(
        UniqueTopic,
        "delete from favo_legis_inits where user_id = $1 and legis_init_id = $2",
        claims.id,
        delegate_favo.vote_result_id
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| UserError::SqlFailure(e))
}
