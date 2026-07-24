use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use common_scrapes::eu_hemicycle::HemicycleLayout;
use reqwest::StatusCode;

use crate::AppState;

pub struct EuHemicycle(pub Arc<HemicycleLayout>);

impl FromRequestParts<AppState> for EuHemicycle {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(EuHemicycle(state.eu_hemicycle.clone()))
    }
}
