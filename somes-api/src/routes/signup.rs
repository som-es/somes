use axum::{extract::State, Json};
use somes_common_lib::{JWTInfo, SignUpInfo};

use crate::{
    db::establish_connection,
    model::NewUser,
    server::{VerificationHasher, VerificationMap}, time::timestamp_secs,
};

use self::{action::validate_signup_info, error::SignUpErrorResponse};

use super::verify::create_verification_id;

mod action;
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
    let id = create_verification_id(verification_hasher, &signup_info);

    let new_user = NewUser::new(
        signup_info.email,
        signup_info.username,
        &signup_info.password,
    )
    .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    // send verification email
    // ...

    // add id to verification map
    verification_map.write().unwrap().insert(id, (new_user, timestamp_secs()));

    // actually insert user after verification (email)
    //insert_user(&mut con, &new_user)?;

    todo!()
}
