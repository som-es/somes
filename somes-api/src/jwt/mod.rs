mod claims;
mod error;
mod keys;

pub use claims::Claims;
pub use error::AuthError;
pub use keys::KEYS;

use axum::Json;
use jsonwebtoken::{encode, Header};
use somes_common_lib::JWTInfo;

pub fn create_access_token(username: String) -> Result<Json<JWTInfo>, AuthError> {
    let claims = Claims::new(username);

    // Create the authorization token
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(JWTInfo { access_token }))
}

#[cfg(test)]
mod tests {
    use super::create_access_token;

    #[test]
    fn test_create_access_token() {
        std::env::set_var("JWT_SECRET", "super_sicher");

        let token = create_access_token("toller_name".to_string()).unwrap();
        let _token = &token.access_token;
    }
}
