use axum::Json;
use somes_common_lib::{JWTInfo, SignUpInfo};

use crate::db::establish_connection;

use self::{error::SignUpError, action::validate_signup_info};

mod error;
mod action;

pub async fn signup(Json(sign_up_info): Json<SignUpInfo>) -> Result<Json<JWTInfo>, SignUpError> {
    let mut con = establish_connection();

    // checks the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &sign_up_info)?;

    // send validation email 
    
    todo!()
}
