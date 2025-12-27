use axum::Json;
use serde_json::json;
use sqlx::query;

use crate::{jwt::Claims, GenericError, PgPoolConnection};

pub async fn delete_account_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<()>, GenericError> {
    let _ = query!("delete from user_topics where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    let _ = query!("delete from favo_dels where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    let _ = query!("delete from favo_legis_inits where user_id = $1", claims.id,)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    query!("delete from somes_user where id = $1", claims.id)
        .execute(&pg)
        .await
        .map(|_| Json(()))
        .map_err(|e| GenericError::SqlFailure(Some(e)))
}
