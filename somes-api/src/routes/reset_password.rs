use axum::Json;
use somes_common_lib::{NewPasswordInfo, ResetPasswordInfo};

use crate::{hash::hash_password, jwt::Claims, operations::user::update_password_hash_at, establish_connection};

pub async fn send_reset_password_request(
    // claims: Claims,
    Json(reset_password_info): Json<ResetPasswordInfo>,
    // update Error
) -> Result<Json<()>, ()> {

    Ok(Json(()))
}

pub async fn reset_password(
    claims: Claims,
    Json(new_password_info): Json<NewPasswordInfo>,
    // update Error
) -> Result<Json<()>, ()> {
    let new_password_hash = hash_password(&new_password_info.password).unwrap();
    let username = claims.sub;

    let mut con = establish_connection();

    // update password in db
    update_password_hash_at(&mut con, &username, &new_password_hash);

    Ok(Json(()))
}
