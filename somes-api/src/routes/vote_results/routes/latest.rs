use axum::Json;
use dataservice::combx::{DbLegislativeInitiativeQuery, OptionalVoteResult};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    routes::{vote_results::construct_vote_result::construct_vote_result, FilterError},
    PgPoolConnection, RedisConnection,
};

#[utoipa::path(
    post,
    path = "/latest_vote_results",
    responses(
        (status = 200, description = "Returned latest vote results successfully.", body = [Vec<OptionalVoteResult>]),
        // (status = 400, description = "Invalid request", body = [LegisInitErrorResponse]),
        // (status = 500, description = "Internal server error", body = [LegisInitErrorResponse])
    )
)]

pub async fn latest_vote_results_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<OptionalVoteResult>>, FilterError> {
    Ok(Json(super::latest_vote_results_sqlx(redis_con, &pg).await?))
}

pub async fn latest_legislative_initiatives_sqlx(
    pg: &PgPool,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives
            where created_at = (select MAX(created_at) from legislative_initiatives
            where accepted is not null) and accepted is not null and is_voteable_on"
    )
    .fetch_all(pg)
    .await?;
    Ok(res)
}

pub async fn latest_vote_results_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
) -> sqlx::Result<Vec<OptionalVoteResult>> {
    futures::future::join_all(
        latest_legislative_initiatives_sqlx(pg)
            .await?
            .into_iter()
            .map(|legis_init| construct_vote_result(redis_con.clone(), pg, legis_init.id))
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<_>>>()
}
