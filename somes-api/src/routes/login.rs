use axum::Json;
use somes_common_lib::{JWTInfo, LoginInfo};

use crate::{
    establish_connection, hash, jwt::{AuthError, create_access_token}, operations::user::get_user_from_db,
};

pub async fn login(Json(login_info): Json<LoginInfo>) -> Result<Json<JWTInfo>, AuthError> {
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

    create_access_token(user.username)
}
