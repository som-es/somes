use axum::Json;
use chrono::{NaiveDate, NaiveDateTime};
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use utoipa::ToSchema;

use crate::{
    routes::{delegate_by_id_sqlx, Delegate, DelegatesErrorResponse},
    PgPoolConnection, RedisConnection,
};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DbMinistrialProposalQueryDelegate {
    pub delegate_id: i32,
    pub ityp: String,
    pub gp: String,
    pub inr: i32,
    pub emphasis: Option<String>,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub due_to: NaiveDate,
    pub ressort: Option<String>,
    pub ressort_shortform: Option<String>,
    pub legis_init_gp: Option<String>,
    pub legis_init_inr: Option<i32>,
    pub legis_init_ityp: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MinistrialProposalDelegate {
    ministrial_proposal: DbMinistrialProposalQueryDelegate,
    delegate: Delegate,
}

pub async fn construct_ministrial_proposal_delegate(
    ministrial_proposal: DbMinistrialProposalQueryDelegate,
    pg: &PgPool,
    redis_con: MultiplexedConnection,
) -> sqlx::Result<MinistrialProposalDelegate> {
    let delegate = delegate_by_id_sqlx(ministrial_proposal.delegate_id, pg, redis_con).await.unwrap();

    Ok(MinistrialProposalDelegate {
        ministrial_proposal,
        delegate,
    })
}

pub async fn extract_latest_ministrial_proposals(
    pg: &PgPool,
    redis_con: MultiplexedConnection,
) -> sqlx::Result<Vec<MinistrialProposalDelegate>> {
    let ministrial_proposals = query_as!(
        DbMinistrialProposalQueryDelegate,
        "select 
        mi.delegate_id,
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
        mp.legis_init_ityp
     from ministrial_issuer as mi 
        inner join ministrial_proposals mp on mp.id = mi.ministrial_proposal_id 
        
        where mp.created_at > NOW() - INTERVAL '14 days'"
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

pub async fn latest_ministrial_proposals(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<MinistrialProposalDelegate>>, DelegatesErrorResponse> {
    extract_latest_ministrial_proposals(&pg, redis_con)
        .await
        .map(Json)
        .map_err(|e| DelegatesErrorResponse::DbSelectFailure(Some(e)))
}
