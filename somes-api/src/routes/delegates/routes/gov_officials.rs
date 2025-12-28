mod decrees;
mod routes;

use axum::{routing::get, Router};
use dataservice::combx::OptionalGovProposal;
pub use decrees::*;
pub use routes::*;

use serde::{Deserialize, Serialize};
use somes_common_lib::{Delegate, ALL_AT_DATE, EXTEND, GOV_PROPOSALS};
use utoipa::ToSchema;

use crate::server::AppState;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct GovProposalDelegate {
    pub gov_proposal: OptionalGovProposal,
    pub delegate: Option<Delegate>,
}

pub fn create_gov_officials_router() -> Router<AppState> {
    Router::new()
        .route(ALL_AT_DATE, get(gov_officials_at_date_route))
        .route(GOV_PROPOSALS, get(gov_proposals_by_official_route))
        .route(EXTEND, get(general_gov_official_info_route))
}
