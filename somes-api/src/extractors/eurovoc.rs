use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use combx::with_data::unique_topics::EurovocTopics;
use reqwest::StatusCode;

use crate::{AppState, TopicsMapper};

pub struct TopicsExtractor(pub Arc<TopicsMapper>);

impl FromRequestParts<AppState> for TopicsExtractor {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(TopicsExtractor(state.topics_mapper.clone()))
    }
}
