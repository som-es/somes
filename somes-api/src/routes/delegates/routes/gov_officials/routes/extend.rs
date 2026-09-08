use axum::{Json, extract::Path};
use combx::{OptionalDecree, OptionalGovProposal};
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};

use crate::{
    PgPoolConnection, RedisConnection, get_json_cache,
    routes::{
        DelegateError, extract_decrees_from_gov_official, extract_gov_proposals_by_delegate_sqlx,
    },
    set_json_cache,
};

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct GeneralGovOfficialInfo {
    pub gov_proposals: Vec<OptionalGovProposal>,
    pub decrees: Vec<OptionalDecree>,
}

pub async fn general_gov_official_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(redis_con): RedisConnection,
    Path(delegate_id): Path<i32>,
) -> Result<Json<GeneralGovOfficialInfo>, DelegateError> {
    Ok(
        extract_general_gov_official_info(delegate_id, &pg, redis_con)
            .await
            .map(Json)?,
    )
}

pub async fn extract_general_gov_official_info(
    delegate_id: i32,
    pg: &PgPool,
    mut redis_con: ConnectionManager,
) -> sqlx::Result<GeneralGovOfficialInfo> {
    let key = format!("general_gov_official_info_{delegate_id}");

    let res = get_json_cache::<GeneralGovOfficialInfo>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let decrees = extract_decrees_from_gov_official(delegate_id, pg).await?;
    let gov_proposals =
        extract_gov_proposals_by_delegate_sqlx(redis_con.clone(), pg, delegate_id).await?;

    let ggoi = GeneralGovOfficialInfo {
        gov_proposals,
        decrees,
    };
    set_json_cache(&mut redis_con, &key, &ggoi).await;
    Ok(ggoi)
}
