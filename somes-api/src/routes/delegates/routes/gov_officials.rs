mod decrees;
mod routes;

use combx::{CombinedData, Delegate, DelegateFilter, OptionalGovProposal};
pub use decrees::*;
pub use routes::*;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;
use serde::{Deserialize, Serialize};
use somes_macro::MeilisearchFilter;
use somes_meilisearch_filter::FilterArgument;
use utoipa::ToSchema;

use combx::OptionalGovProposalFilter;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize, MeilisearchFilter)]
pub struct GovProposalDelegate {
    #[filter(make_optional)]
    pub gov_proposal: OptionalGovProposal,
    pub delegates: Option<Vec<Delegate>>,
}

impl CombinedData for GovProposalDelegate {
    const INDEX: combx::Index = OptionalGovProposal::INDEX;

    const PRIMARY_KEY: &str = OptionalGovProposal::PRIMARY_KEY;

    fn id(&self) -> u64 {
        self.gov_proposal.id.unwrap() as u64
    }
}

pub fn create_gov_officials_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(gov_officials_at_date_route))
        .routes(routes!(gov_proposals_by_official_route))
        .routes(routes!(general_gov_official_info_route))
}
