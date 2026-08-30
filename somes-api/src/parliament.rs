use axum::{extract::FromRequestParts, http::request::Parts};

pub use combx::Parliament;

#[derive(Debug, Clone, Copy)]
pub struct ParliamentCtx(pub Parliament);

impl<S: Send + Sync> FromRequestParts<S> for ParliamentCtx {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let parliament = parts
            .extensions
            .get::<Parliament>()
            .copied()
            .unwrap_or_default();
        Ok(Self(parliament))
    }
}
