use axum::{extract::Query, Json};
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::DelegateById;
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};

use crate::{
    get_json_cache,
    routes::{
        extract_decrees_from_gov_official, extract_gov_prosals_by_delegate, DelegatesErrorResponse, GovProposal
    }, set_json_cache, PgPoolConnection, RedisConnection,
};

use super::Decree;

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct GeneralGovOfficialInfo {
    pub gov_proposals: Vec<GovProposal>,
    pub decrees: Vec<Decree>,
}

pub async fn extract_general_gov_official_info(
    delegate_id: i32,
    pg: &PgPool,
    redis_con: &mut MultiplexedConnection,
) -> sqlx::Result<GeneralGovOfficialInfo> {
    let key = format!("general_gov_official_info_{delegate_id}");

    let res = get_json_cache::<GeneralGovOfficialInfo>(redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let decrees = extract_decrees_from_gov_official(delegate_id, pg).await?;
    let gov_proposals = extract_gov_prosals_by_delegate(redis_con.clone(), pg, delegate_id).await?;

    let ggoi = GeneralGovOfficialInfo {
        gov_proposals,
        decrees,
    };
    set_json_cache(redis_con, &key, &ggoi).await;
    Ok(ggoi)
}
 
pub async fn general_gov_official_info(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<GeneralGovOfficialInfo>, DelegatesErrorResponse> {
    extract_general_gov_official_info(delegate_by_id.delegate_id, &pg, &mut redis_con)
        .await
        .map(Json)
        .map_err(|e| DelegatesErrorResponse::DbSelectFailure(Some(e)))
}
