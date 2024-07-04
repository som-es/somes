use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use somes_common_lib::time::timestamp_secs;
use utoipa::IntoParams;

use super::{error::AuthError, keys::KEYS};

#[derive(IntoParams, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(id: i32, sub: String) -> Self {
        Self {
            id,
            sub,
            company: "".to_string(),
            // Mandatory expiry time as UNIX timestamp
            exp: (timestamp_secs() + 60 * 60 * 24 * 3) as usize,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
