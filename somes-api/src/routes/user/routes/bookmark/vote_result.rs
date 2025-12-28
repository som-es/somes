use axum::Json;
use somes_common_lib::LegisInitFavo;
use sqlx::query_as;

use crate::{jwt::Claims, routes::UserError, PgPoolConnection};

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
