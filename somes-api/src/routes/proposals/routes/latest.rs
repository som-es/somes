use axum::{extract::Query, Json};
use dataservice::db::models::DbMinistrialProposalQueryMeta;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use utoipa::ToSchema;

use crate::{
    routes::{construct_gov_delegate_proposal, FilterError, GovProposalDelegate},
    PgPoolConnection, RedisConnection,
};

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Days {
    days: u32,
}

pub async fn latest_gov_proposals_route(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(days): Query<Days>,
) -> Result<Json<Vec<GovProposalDelegate>>, FilterError> {
    if days.days > 180 {
        return Err(FilterError::InvalidDays(days.days as u32));
    }

    Ok(
        extract_latest_ministrial_proposals(&pg, redis_con, days.days as i32)
            .await
            .map(Json)?,
    )
}

pub async fn extract_latest_ministrial_proposals(
    pg: &PgPool,
    redis_con: MultiplexedConnection,
    days: i32,
) -> sqlx::Result<Vec<GovProposalDelegate>> {
    let ministrial_proposals = query_as!(
        DbMinistrialProposalQueryMeta,
        "select
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
         ministrial_proposals mp 

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
                construct_gov_delegate_proposal(redis_con.clone(), pg, ministrial_proposal)
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<_>>>()
}
