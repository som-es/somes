use axum::{extract::Path, Json};
use dataservice::combx::OptionalVoteResult;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    routes::{vote_results::construct_vote_result::construct_vote_result, FilterError},
    PgPoolConnection, RedisConnection,
};

pub async fn vote_result_by_id_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path(vote_result_id): Path<i32>,
) -> Result<Json<OptionalVoteResult>, FilterError> {
    vote_result_by_id_sqlx(redis_con, &pg, vote_result_id)
        .await?
        .ok_or(FilterError::NotFound)
        .map(Json)
}

pub async fn vote_result_by_id_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Option<OptionalVoteResult>> {
    construct_vote_result(redis_con.clone(), pg, legis_init_id).await
}
