use axum::{extract::Path, Json};
use dataservice::combx::DbMinistrialProposalQueryMeta;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    routes::{construct_gov_delegate_proposal, GovProposalDelegate, LegisInitErrorResponse},
    PgPoolConnection, RedisConnection,
};

pub async fn gov_proposal_by_path(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, inr)): Path<(String, i32)>,
) -> Result<Json<GovProposalDelegate>, LegisInitErrorResponse> {
    gov_proposal_delegate_by_path_sqlx(redis_con, &pg, &gp, inr)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::VoteResultById)
}

pub async fn gov_proposal_delegate_by_path_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    inr: i32,
) -> sqlx::Result<GovProposalDelegate> {
    let miniserial_prop = sqlx::query_as!(
        DbMinistrialProposalQueryMeta,
        "
            select
                mi.delegate_id,
                mp.id,
                mp.ityp,
                mp.gp,
                mp.inr,
                mp.emphasis,
                mp.title,
                mp.description,
                mp.created_at,
                mp.updated_at,
                mp.due_to,
                mp.ressort,
                mp.ressort_shortform,
                mp.legis_init_gp,
                mp.legis_init_inr,
                mp.legis_init_ityp,
                mp.has_vote_result
            from
                ministrial_issuer as mi
            inner join
                ministrial_proposals as mp
            on
                mp.id = mi.ministrial_proposal_id
            where
                gp = $1 and inr = $2
        ",
        gp,
        inr
    )
    .fetch_one(pg)
    .await?;
    construct_gov_delegate_proposal(redis_con.clone(), pg, miniserial_prop).await
}
