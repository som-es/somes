mod all_props;
mod construct_gov_proposal;
mod db;
mod routes;
pub use all_props::*;
pub use construct_gov_proposal::*;
pub use db::*;
pub use routes::*;

use axum::{Router, routing::get};
use combx::models::DbMinistrialProposalQueryMeta;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use somes_common_lib::{LATEST, SEARCH};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::AppState;

use super::{GovProposalDelegate, delegate_by_id_sqlx};

pub fn create_gov_proposals_router() -> Router<AppState> {
    Router::new()
        .route(SEARCH, get(gov_props_by_search_route))
        // .route(LIVE, post(gov_proposals_per_page_route))
        .route(LATEST, get(latest_gov_proposals_route))
        .route("/{gp}/{inr}", get(gov_proposal_by_path_route))
        .nest("/{gp}/{inr}/mood", create_proposal_mood_router())
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct GovProposalsWithMaxPage {
    pub gov_proposals: Vec<GovProposalDelegate>,
    pub entry_count: i64,
    pub max_page: i64,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub async fn construct_gov_delegate_proposal(
    mut redis_con: ConnectionManager,
    pg: &PgPool,
    ministrial_proposal: DbMinistrialProposalQueryMeta,
) -> sqlx::Result<GovProposalDelegate> {
    let gov_proposal = construct_gov_proposal(redis_con.clone(), &pg, ministrial_proposal).await?;
    let mut delegates = vec![];
    for ministerial_issuer in gov_proposal.ministerial_issuers.as_deref().unwrap_or(&[]) {
        let delegate = delegate_by_id_sqlx(*ministerial_issuer, &pg, &mut redis_con).await?;
        delegates.push(delegate);
    }
    Ok(GovProposalDelegate {
        gov_proposal,
        delegates: Some(delegates),
    })
}
