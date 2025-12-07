use axum::{extract::Path, Json};
use dataservice::combx::{DbLegislativeInitiativeQuery, OptionalVoteResult};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    routes::{vote_results::construct_vote_result::construct_vote_result, LegisInitErrorResponse},
    PgPoolConnection, RedisConnection,
};

pub async fn vote_result_by_path(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, ityp, inr)): Path<(String, String, i32)>,
) -> Result<Json<OptionalVoteResult>, LegisInitErrorResponse> {
    vote_result_by_path_sqlx(redis_con, &pg, &gp, &ityp, inr)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn vote_result_by_path_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<OptionalVoteResult> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where gp = $1 and ityp = $2 and inr = $3",
        gp,
        ityp,
        inr
    )
    .fetch_one(pg)
    .await?;
    construct_vote_result(redis_con.clone(), pg, legis_init.id).await
}
