use axum::Json;
use somes_common_lib::{JWTInfo, LoginInfo};

use crate::{
    establish_connection, hash,
    jwt::{create_access_token, AuthError},
    operations::user::get_user_from_db,
};

pub async fn login(Json(login_info): Json<LoginInfo>) -> Result<Json<JWTInfo>, AuthError> {
    // mitigate brute force attacks
    // Start showing CAPTCHAs after three incorrect attempts from an IP
    // After an incorrect attempt, block all new login requests to your server from that IP for a period of time. Increment this on every failed attempt.
    // Keep a log and note spikes of activity. If someone is trying to bruteforce, you ought to make note of that and counter it.

    let con = &mut establish_connection();

    let user = get_user_from_db(
        con,
        &login_info.username_or_email,
        &login_info.username_or_email,
    )
    .ok_or(AuthError::WrongCredentials)?;

    if !hash::verify_password(&login_info.password, &user.password_hash)
        .map_err(|_| AuthError::WrongCredentials)?
    {
        return Err(AuthError::WrongCredentials);
    }

    create_access_token(user.id, user.username)
}
