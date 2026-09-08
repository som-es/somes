use axum::Json;
use reqwest::StatusCode;
use somes_common_lib::DelegateFavo;
use sqlx::query_as;

use crate::{PgPoolConnection, jwt::Claims, routes::UserError};

pub async fn add_user_delegate_bookmark(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<DelegateFavo>,
) -> Result<Json<()>, UserError> {
    if delegate_favo.user_info_days.is_negative() {
        return Err(UserError::Custom(
            StatusCode::BAD_REQUEST,
            "invalid info user days".into(),
        ));
    }
    query_as!(
        UniqueTopic,
        "insert into favo_dels(user_id, delegate_id, user_info_days) values ($1, $2, $3) on conflict do nothing",
        claims.id,
        delegate_favo.delegate_id,
        delegate_favo.user_info_days as i32
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| UserError::SqlFailure(e))
}

pub async fn update_user_delegate_bookmark(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<DelegateFavo>,
) -> Result<Json<()>, UserError> {
    if delegate_favo.user_info_days.is_negative() {
        return Err(UserError::Custom(
            StatusCode::BAD_REQUEST,
            "invalid info user days".into(),
        ));
    }
    query_as!(
        UniqueTopic,
        "update favo_dels set user_info_days = $3 where user_id = $1 and delegate_id = $2",
        claims.id,
        delegate_favo.delegate_id,
        delegate_favo.user_info_days as i32
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| UserError::SqlFailure(e))
}

pub async fn delegate_bookmarks_by_user(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<DelegateFavo>>, UserError> {
    query_as!(
        DelegateFavo,
        "select delegate_id, user_info_days from favo_dels where user_id = $1",
        claims.id,
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| UserError::SqlFailure(e))
}

pub async fn remove_user_delegate_bookmark(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<DelegateFavo>,
) -> Result<Json<()>, UserError> {
    query_as!(
        UniqueTopic,
        "delete from favo_dels where user_id = $1 and delegate_id = $2",
        claims.id,
        delegate_favo.delegate_id
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| UserError::SqlFailure(e))
}
