use axum::Json;
use serde_json::json;
use somes_common_lib::LegisInitFavo;
use sqlx::query_as;

use crate::{jwt::Claims, PgPoolConnection};

pub async fn add_legis_init_favo(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<LegisInitFavo>,
) -> Result<Json<()>, Json<serde_json::Value>> {
    query_as!(
        UniqueTopic,
        "insert into favo_legis_inits(user_id, legis_init_id) values ($1, $2) on conflict do nothing",
        claims.id,
        delegate_favo.vote_result_id,
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|_| Json(json!({"error": "db error"})))
}

pub async fn user_legis_init_favos(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<LegisInitFavo>>, Json<serde_json::Value>> {
    query_as!(
        LegisInitFavo,
        "select legis_init_id as vote_result_id from favo_legis_inits where user_id = $1",
        claims.id,
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "db error"})))
}

pub async fn remove_user_legis_init_favo(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<LegisInitFavo>,
) -> Result<Json<()>, Json<serde_json::Value>> {
    query_as!(
        UniqueTopic,
        "delete from favo_legis_inits where user_id = $1 and legis_init_id = $2",
        claims.id,
        delegate_favo.vote_result_id
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|_| Json(json!({"error": "db error"})))
}
