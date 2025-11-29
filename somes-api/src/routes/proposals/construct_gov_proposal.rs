use dataservice::combx::{DbMinistrialProposalQuery, GovProposal};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    get_json_cache,
    routes::{
        get_vote_result_by_unique_hints_with_accepted_required,
        proposals::db::sqlx_get_eurovoc_topics_from_ministrial_proposal,
        sqlx_get_docs_from_ministerial_prop, sqlx_get_ministerial_issuers,
    },
    today,
};

pub async fn construct_gov_proposal(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    ministrial_proposal: DbMinistrialProposalQuery,
) -> sqlx::Result<GovProposal> {
    let key = format!("ministrial_prop/{}", ministrial_proposal.id);
    let res = get_json_cache::<GovProposal>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let vote_result = match (
        &ministrial_proposal.legis_init_gp,
        &ministrial_proposal.legis_init_ityp,
        ministrial_proposal.legis_init_inr,
    ) {
        (Some(ref gp), Some(ref ityp), Some(ref inr)) => {
            get_vote_result_by_unique_hints_with_accepted_required(
                redis_con.clone(),
                &pg,
                &gp,
                &ityp,
                *inr,
            )
            .await
            .expect(&format!("{gp}/{ityp}/{inr}: {ministrial_proposal:?}")) // return proper error
        }
        _ => None,
    };
    let gov_proposal = GovProposal {
        vote_result,
        eurovoc_topics: sqlx_get_eurovoc_topics_from_ministrial_proposal(
            pg,
            ministrial_proposal.id,
        )
        .await?,
        topics: sqlx_get_eurovoc_topics_from_ministrial_proposal(pg, ministrial_proposal.id)
            .await?,
        other_keyword_topics: sqlx_get_eurovoc_topics_from_ministrial_proposal(
            pg,
            ministrial_proposal.id,
        )
        .await?,
        ministerial_issuers: sqlx_get_ministerial_issuers(pg, ministrial_proposal.id).await?,
        documents: sqlx_get_docs_from_ministerial_prop(pg, ministrial_proposal.id).await?,
        ministrial_proposal,
    };

    let cache_date = if let Some(ref vote_result) = gov_proposal.vote_result {
        vote_result
            .legislative_initiative
            .as_ref()
            .unwrap()
            .created_at
            .unwrap()
    } else {
        today()
    };
    crate::set_json_cache_with_relevance(&mut redis_con, &key, &gov_proposal, cache_date)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;

    Ok(gov_proposal)
}
