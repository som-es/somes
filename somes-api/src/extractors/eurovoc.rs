use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use combx::with_data::unique_topics::EurovocTopics;
use reqwest::StatusCode;

use crate::AppState;

pub struct Eurovoc(pub Arc<EurovocTopics>);

impl FromRequestParts<AppState> for Eurovoc {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Eurovoc(state.eurovoc_topics.clone()))
    }
}
