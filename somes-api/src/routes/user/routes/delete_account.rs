use axum::Json;
use serde_json::json;
use sqlx::query;

use crate::{jwt::Claims, PgPoolConnection};

pub async fn delete_account_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<()>, Json<serde_json::Value>> {
    let _ = query!("delete from user_topics where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|_| Json(json!({"error": "db error"})))?;

    let _ = query!("delete from favo_dels where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|_| Json(json!({"error": "db error"})))?;

    let _ = query!("delete from favo_legis_inits where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|_| Json(json!({"error": "db error"})))?;

    query!("delete from somes_user where id = $1", claims.id)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|_| Json(json!({"error": "db error"})))
}
