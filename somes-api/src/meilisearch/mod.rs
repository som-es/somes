mod decrees;
mod delegate_questions;
mod delegates;
mod gov_props;
mod party;
mod refresh;
mod swap;
mod vote_results;

pub mod update_time;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use meilisearch_sdk::settings::{PaginationSetting, Settings};
use reqwest::StatusCode;

use crate::AppState;

pub use combx::Index;
pub use decrees::create_or_update_decrees_meilisearch_index;
pub use delegate_questions::update_delegate_questions_meilisearch_index;
pub use delegates::update_delegates_meilisearch_index;
pub use gov_props::create_or_update_gov_props_meilisearch_index;
pub use party::{party_of_delegate_at_time, party_of_delegates_at_time};
pub use refresh::update_meilisearch_indices;
pub use update_time::*;
pub use vote_results::update_vote_result_meilisearch_index;

pub(crate) const RANKING_RULES: [&str; 6] = [
    "sort",
    "words",
    "typo",
    "proximity",
    "attribute",
    "exactness",
];
pub(crate) const MAX_TOTAL_HITS: usize = 100_000_000;

pub(crate) fn index_settings(
    filterable_attributes: &[&str],
    sortable_attributes: &[&str],
) -> Settings {
    Settings::new()
        .with_ranking_rules(RANKING_RULES)
        .with_filterable_attributes(filterable_attributes.iter().copied())
        .with_sortable_attributes(sortable_attributes.iter().copied())
        .with_pagination(PaginationSetting {
            max_total_hits: MAX_TOTAL_HITS,
        })
}

#[derive(FromRef)]
pub struct MeilisearchClient(pub meilisearch_sdk::client::Client);

impl FromRequestParts<AppState> for MeilisearchClient {
    type Rejection = (StatusCode, String);

    #[inline]
    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.meilisearch_client.clone()))
    }
}
