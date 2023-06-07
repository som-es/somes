use axum::Json;
use somes_common_lib::{NewPasswordInfo, ResetPasswordInfo};

use crate::{hash::hash_password, jwt::Claims};

pub async fn send_reset_password_request(
    // claims: Claims,
    Json(reset_password_info): Json<ResetPasswordInfo>,
) -> Result<Json<()>, ()> {
    // update Error

    Ok(Json(()))
}

pub async fn reset_password(
    claims: Claims,
    Json(new_password_info): Json<NewPasswordInfo>,
) -> Result<Json<()>, ()> {
    // update Error
    let password_hash = hash_password(&new_password_info.password).unwrap();
    Ok(Json(()))
}
