use axum::{extract::FromRequestParts, http::request::Parts};
use reqwest::StatusCode;

pub struct Qs<T>(pub T);

impl<S, T> FromRequestParts<S> for Qs<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or("");
        let config = serde_qs::Config::new(5, false);

        let value = config
            .deserialize_str(query)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Qs(value))
    }
}
