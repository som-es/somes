mod claims;
mod error;
mod keys;

pub use claims::Claims;
pub use error::AuthError;
pub use keys::KEYS;

use axum::Json;
use somes_common_lib::JWTInfo;
use jsonwebtoken::{encode, Header};

pub fn create_access_token(username: String) -> Result<Json<JWTInfo>, AuthError> {
    let claims = Claims::new(username);

    // Create the authorization token
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(JWTInfo { access_token }))
}