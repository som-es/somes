mod claims;
mod error;
mod keys;

pub use claims::*;
pub use error::AuthError;
pub use keys::*;

use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use somes_common_lib::{time::timestamp_secs, JWTInfo};

pub fn create_access_token(
    id: i32,
    username: String,
    is_admin: bool,
) -> Result<Json<JWTInfo>, AuthError> {
    create_access_token_with_keys_and_exp_time(
        id,
        username,
        is_admin,
        (timestamp_secs() + 60 * 60 * 24 * 3) as usize,
        &KEYS.encoding,
    )
}

pub fn create_access_token_u128(
    id: u128,
    username: String,
    is_admin: bool,
    key: &EncodingKey,
) -> Result<Json<JWTInfo>, AuthError> {
    create_access_token_with_keys_and_exp_time(
        id,
        username,
        is_admin,
        (timestamp_secs() + 60 * 60 * 3) as usize,
        key,
    )
}

pub fn create_access_token_with_keys_and_exp_time<T: Serialize>(
    id: T,
    username: String,
    is_admin: bool,
    exp_secs: usize,
    key: &EncodingKey,
) -> Result<Json<JWTInfo>, AuthError> {
    // let claims = ClaimsGen::<T>::new(id, username, is_admin);
    let claims = ClaimsGen {
        id,
        sub: username,
        company: "".to_string(),
        // Mandatory expiry time as UNIX timestamp
        exp: exp_secs,
        is_admin,
    };

    // Create the authorization token
    let access_token =
        encode(&Header::default(), &claims, key).map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(JWTInfo { access_token }))
}

pub async fn renew_token(claims: Claims) -> Result<Json<JWTInfo>, AuthError> {
    create_access_token(claims.id, claims.sub, claims.is_admin)
}

#[cfg(test)]
mod tests {
    use super::create_access_token;

    #[test]
    fn test_create_access_token() {
        std::env::set_var("JWT_SECRET", "super_sicher");

        let token = create_access_token(43, "toller_name".to_string(), false).unwrap();
        let _token = &token.access_token;
    }
}
