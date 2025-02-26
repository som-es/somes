use dataservice::db::models::DbMinistrialProposalQuery;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::routes::GovProposalDelegate;

use super::construct_gov_delegate_proposal;

pub async fn get_all_gov_props(
    redis_con: MultiplexedConnection,
    con: &PgPool,
) -> sqlx::Result<Vec<GovProposalDelegate>> {
    let ministrial_props = sqlx::query_as!(
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
        "
    )
    .fetch_all(con)
    .await?;

    let mut gov_props_dels = Vec::with_capacity(ministrial_props.len());

    for ministrial_prop in ministrial_props {
        match construct_gov_delegate_proposal(redis_con.clone(), con, ministrial_prop).await {
            Ok(vote_result) => gov_props_dels.push(vote_result),
            Err(e) => {
                log::warn!("Error while constructing vote result, skipped in result of it: {e:?}")
            }
        }
    }

    Ok(gov_props_dels)
}
