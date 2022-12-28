use axum::{extract::State, Json};
use somes_common_lib::{JWTInfo, SignUpInfo};

use crate::{
    db::establish_connection,
    server::VerificationMap,
};

pub use self::{action::{validate_signup_info, add_new_user_to_verification_map}, error::SignUpErrorResponse};

mod action;
mod error;

pub async fn signup(
    State(verification_map): State<VerificationMap>,
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<JWTInfo>, SignUpErrorResponse> {
    let mut con = establish_connection();

    // checks the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &signup_info, verification_map.clone())?;
    
    add_new_user_to_verification_map(signup_info, verification_map)?;

    
    todo!()
}

