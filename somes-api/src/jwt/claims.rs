use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use somes_common_lib::time::timestamp_secs;
use utoipa::IntoParams;

use super::{error::AuthError, keys::KEYS};

pub type Claims = ClaimsGen<i32>;

#[derive(IntoParams, Debug, Serialize, Deserialize)]
pub struct ClaimsGen<T> {
    pub id: T,
    pub sub: String,
    pub company: String,
    pub exp: usize,
    pub is_admin: bool,
}

impl<T> ClaimsGen<T> {
    pub fn new(id: T, sub: String, is_admin: bool) -> Self {
        Self {
            id,
            sub,
            company: "".to_string(),
            // Mandatory expiry time as UNIX timestamp
            exp: (timestamp_secs() + 60 * 60 * 24 * 3) as usize,
            is_admin,
        }
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for ClaimsGen<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;
        // Decode the user data
        let token_data =
            decode::<ClaimsGen<T>>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, Validation};

    use crate::{jwt::KEYS, AuthError};

    #[test]
    fn test_jwt_decode() {
        std::env::set_var("JWT_SECRET", "asdfasdfsdf");
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NCwic3ViIjoiZmxvcmlhbi5uYWd5QGl0Lmh0bC1obC5hYy5hdCIsImNvbXBhbnkiOiIiLCJleHAiOjE3MzIyOTkzNzh9.nObVpb4lPKzy85Ki_f8Z1VtGoz37BQq2u9L0crgUBrI";

        use crate::jwt::Claims;

        let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)
            .unwrap();
    }
}
