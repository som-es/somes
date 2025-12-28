use axum::{
    extract::{Path, Query},
    Json,
};
use redis::aio::MultiplexedConnection;
use somes_common_lib::FullMandate;
use somes_common_lib::{Delegate, DelegateById};
use sqlx::PgPool;

use crate::{get_json_cache, routes::DelegateError, PgPoolConnection, RedisConnection};

pub async fn delegate_by_id_sqlx(
    delegate_id: i32,
    pg: &PgPool,
    mut redis_con: MultiplexedConnection,
) -> sqlx::Result<Delegate> {
    let key = delegate_id.to_string();
    let res = get_json_cache::<Delegate>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }
    let delegate = sqlx::query_as!(
        Delegate,
        "
        SELECT
            * from delegates_with_mandates d
        WHERE
        d.id = $1;
    ",
        delegate_id
    )
    .fetch_one(pg)
    .await?;

    crate::set_json_cache(&mut redis_con, &key, &delegate)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;

    Ok(delegate)
}

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate",
    responses(
        (status = 200, description = "Returned delegate successfully.", body = [Delegate]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegate_by_id(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Delegate>, DelegateError> {
    Ok(
        delegate_by_id_sqlx(delegate_by_id.delegate_id, &pg, redis_con)
            .await
            .map(Json)?,
    )
}

pub async fn delegate_by_id_path_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path(id): Path<i32>,
) -> Result<Json<Delegate>, DelegateError> {
    Ok(delegate_by_id_sqlx(id, &pg, redis_con).await.map(Json)?)
}
