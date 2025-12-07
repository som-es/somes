use axum::{extract::Query, Json};
use dataservice::combx::OptionalVoteResult;
use redis::aio::MultiplexedConnection;
use somes_common_lib::{Page, VoteResultById};
use sqlx::PgPool;

use crate::{
    routes::{vote_results::construct_vote_result::construct_vote_result, LegisInitErrorResponse},
    PgPoolConnection, RedisConnection,
};

#[utoipa::path(
    post,
    path = "/vote_result_by_id",
    params
    (
        Page
    ),
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<OptionalVoteResult>]),
        // (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        // (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]
pub async fn vote_result_by_id_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(vote_result_id): Query<VoteResultById>,
) -> Result<Json<OptionalVoteResult>, LegisInitErrorResponse> {
    vote_result_by_id_sqlx(redis_con, &pg, vote_result_id.id)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_result_by_id_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<OptionalVoteResult> {
    construct_vote_result(redis_con.clone(), pg, legis_init_id).await
}
