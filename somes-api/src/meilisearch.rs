use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use reqwest::StatusCode;

use crate::server::AppState;

#[derive(FromRef)]
pub struct MeilisearchClient(pub meilisearch_sdk::client::Client);


#[async_trait]
impl FromRequestParts<AppState> for MeilisearchClient
where
{
    type Rejection = (StatusCode, String);

    #[inline]
    async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        Ok(Self(state.meilisearch_client.clone()))
    }
}
