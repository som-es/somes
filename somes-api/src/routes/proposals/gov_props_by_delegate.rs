use axum::{
    extract::{Path, Query},
    Json,
};
use dataservice::combx::GovProposal;
use somes_common_lib::DelegateById;
use sqlx::query_as;

use crate::{
    routes::{construct_gov_proposal, LegisInitErrorResponse},
    PgPoolConnection, RedisConnection,
};

pub async fn gov_proposals_by_official(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path(delegate_id): Path<i32>,
) -> Result<Json<Vec<GovProposal>>, LegisInitErrorResponse> {
    extract_gov_prosals_by_delegate(redis_con, &pg, delegate_id)
        .await
        .map(Json)
        .map_err(|_| LegisInitErrorResponse::LegisInit)
}

pub async fn extract_gov_prosals_by_delegate(
    redis_con: redis::aio::MultiplexedConnection,
    pg: &sqlx::Pool<sqlx::Postgres>,
    delegate_id: i32,
) -> sqlx::Result<Vec<GovProposal>> {
    use dataservice::db::models::DbMinistrialProposalQuery;
    let ministrial_proposals = query_as!(
        DbMinistrialProposalQuery,
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
            delegate_id = $1
        order by created_at DESC;
    ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    /*ministrial_proposals.into_iter().map(|ministrial_proposal| {
        match (
            &ministrial_proposal.legis_init_gp,
            &ministrial_proposal.legis_init_ityp,
            ministrial_proposal.legis_init_inr,
        ) {
            (Some(ref gp), Some(ref ityp), Some(ref inr)) => {
                Some(
                    get_vote_result_by_unique_hints(redis_con.clone(), &pg, gp, ityp, *inr)
                )
            }
            _ => None,
        }
    }).collect::<Vec<_>>();*/

    // let mut gov_proposal_futures = Vec::with_capacity(ministrial_proposals.len());

    futures::future::join_all(ministrial_proposals.into_iter().map(|ministrial_proposal| {
        let redis_con = redis_con.clone();
        construct_gov_proposal(redis_con, &pg, ministrial_proposal)
    }))
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<_>>>()
}
