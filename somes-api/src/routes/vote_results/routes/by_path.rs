use axum::{Json, extract::Path};
use combx::{DbLegislativeInitiativeQuery, OptionalVoteResult};
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::{
    PgPoolConnection, RedisConnection,
    routes::{FilterError, vote_results::construct_vote_result::construct_vote_result},
};

#[utoipa::path(
    get,
    path = "/{gp}/{ityp}/{inr}",
    tag = "vote_results",
    params(("gp" = String, Path, description = "Legislative period"), ("ityp" = String, Path, description = "Initiative type"), ("inr" = i32, Path, description = "Initiative number")),
    responses(
        (status = 200, description = "Vote result by path", body = OptionalVoteResult),
    )
)]
pub async fn vote_result_by_path_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, ityp, inr)): Path<(String, String, i32)>,
) -> Result<Json<OptionalVoteResult>, FilterError> {
    vote_result_by_path_sqlx(redis_con, &pg, &gp, &ityp, inr)
        .await?
        .ok_or(FilterError::NotFound)
        .map(Json)
}

pub async fn vote_result_by_path_sqlx(
    redis_con: ConnectionManager,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<Option<OptionalVoteResult>> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where gp = $1 and ityp = $2 and inr = $3",
        gp,
        ityp,
        inr
    )
    .fetch_one(pg)
    .await?;
    construct_vote_result(redis_con, pg, legis_init.id).await
}
