use axum::{extract::State, Json};
use somes_common_lib::{JWTInfo, SignUpInfo};

use crate::{db::establish_connection, model::NewUser, server::{VerificationMap, VerificationHasher}};

use self::{action::validate_signup_info, db::insert_user, error::SignUpErrorResponse};

use super::verify::create_verification_id;

mod action;
mod db;
mod error;

pub async fn signup(
    State(verification_map): State<VerificationMap>,
    State(verification_hasher): State<VerificationHasher>,
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<JWTInfo>, SignUpErrorResponse> {
    let mut con = establish_connection();

    // checks the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &signup_info, verification_map.clone())?;
    
    // create an (hopefully) unique verification id
    let hash = create_verification_id(verification_hasher, &signup_info);

    // send verification email and add to verification map
    let new_user = NewUser::new(
        signup_info.email,
        signup_info.username,
        &signup_info.password,
    )
    .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    verification_map.write().unwrap().insert(hash, new_user);

    // actually insert user after verification
    //insert_user(&mut con, &new_user)?;

    todo!()
}
