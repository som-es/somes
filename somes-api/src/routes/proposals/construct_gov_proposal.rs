use dataservice::combx::{models::*, DbMinistrialProposalQueryMeta, VoteResult};
use redis::aio::MultiplexedConnection;
use somes_common_lib::Document;
use sqlx::PgPool;

use crate::{
    get_json_cache,
    today,
};

pub async fn get_gov_proposal_sqlx(pg: &PgPool, id: i32) -> sqlx::Result<OptionalGovProposal> {
    sqlx::query_as!(
        OptionalGovProposal,
        "select * from gov_proposals where id = $1",
        id
    )
    .fetch_one(pg)
    .await
}

pub async fn construct_gov_proposal(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    ministrial_proposal: DbMinistrialProposalQueryMeta,
) -> sqlx::Result<OptionalGovProposal> {
    let key = format!("ministrial_prop/{}", ministrial_proposal.id);
    let res = get_json_cache::<OptionalGovProposal>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let gov_proposal = get_gov_proposal_sqlx(pg, ministrial_proposal.id).await?;

    let cache_date = if let Some(ref vote_result) = gov_proposal.vote_result {
        vote_result.legislative_initiative.created_at
    } else {
        today()
    };
    crate::set_json_cache_with_relevance(&mut redis_con, &key, &gov_proposal, cache_date)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;

    Ok(gov_proposal)
}
