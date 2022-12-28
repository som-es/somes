use axum::Json;
use somes_common_lib::{JWTInfo, SignUpInfo};

use crate::db::establish_connection;

use self::{
    action::validate_signup_info,
    error::SignUpErrorResponse, db::insert_new_user,
};

mod action;
mod error;
mod db;

pub async fn signup(
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<JWTInfo>, SignUpErrorResponse> {
    let mut con = establish_connection();

    // checks the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &signup_info)?;

    // send verification email

    // actually insert user after verification
    insert_new_user(&mut con, &signup_info)?;

    todo!()
}
