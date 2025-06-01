use axum::{extract::Query, Json};
use chrono::{NaiveDate, NaiveDateTime};
use dataservice::db::models::DbMinistrialProposalQuery;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::Delegate;
use sqlx::{query_as, PgPool};
use utoipa::ToSchema;

use crate::{
    routes::{construct_gov_proposal, delegate_by_id_sqlx, DelegatesErrorResponse, GovProposal},
    PgPoolConnection, RedisConnection,
};

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct GovProposalDelegate {
    pub gov_proposal: GovProposal,
    pub delegate: Delegate,
}

pub async fn construct_ministrial_proposal_delegate(
    ministrial_proposal: DbMinistrialProposalQuery,
    pg: &PgPool,
    redis_con: MultiplexedConnection,
) -> sqlx::Result<GovProposalDelegate> {
    let delegate =
        delegate_by_id_sqlx(ministrial_proposal.delegate_id, pg, redis_con.clone()).await?;
    let gov_proposal = construct_gov_proposal(redis_con, &pg, ministrial_proposal).await?;

    Ok(GovProposalDelegate {
        gov_proposal,
        delegate,
    })
}

pub async fn extract_latest_ministrial_proposals(
    pg: &PgPool,
    redis_con: MultiplexedConnection,
    days: i32,
) -> sqlx::Result<Vec<GovProposalDelegate>> {
    let ministrial_proposals = query_as!(
        DbMinistrialProposalQuery,
        "select 
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
     from ministrial_issuer as mi 
        inner join ministrial_proposals mp on mp.id = mi.ministrial_proposal_id 
        
        where mp.created_at > NOW() - make_interval(days => $1)
    order by mp.created_at desc",
        days
    )
    .fetch_all(pg)
    .await?;

    futures::future::join_all(
        ministrial_proposals
            .into_iter()
            .map(|ministrial_proposal| {
                construct_ministrial_proposal_delegate(ministrial_proposal, pg, redis_con.clone())
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<_>>>()
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Days {
    days: u32,
}

pub async fn latest_ministrial_proposals(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(days): Query<Days>,
) -> Result<Json<Vec<GovProposalDelegate>>, DelegatesErrorResponse> {
    if days.days > 180 {
        return Err(DelegatesErrorResponse::Custom(
            "days cannot be larger than 180".to_string(),
        ));
    }

    extract_latest_ministrial_proposals(&pg, redis_con, days.days as i32)
        .await
        .map(Json)
        .map_err(|e| DelegatesErrorResponse::DbSelectFailure(Some(e)))
}
