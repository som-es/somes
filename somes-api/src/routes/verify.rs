use std::{
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

use axum::extract::{Query, State};
use somes_common_lib::{JWTInfo, SignUpInfo, VerificationIDInfo};
use uuid::Uuid;

use crate::{
    establish_connection,
    operations::user::insert_user,
    server::{VerificationHasher, VerificationMap},
};

use self::error::VerifyErrorResponse;

mod error;

pub fn create_verification_id(hasher: VerificationHasher, signup_info: &SignUpInfo) -> u64 {
    let mut bytes = Uuid::new_v4().as_bytes().to_vec();
    bytes.extend_from_slice(signup_info.email.as_bytes());
    bytes.extend_from_slice(signup_info.username.as_bytes());

    bytes.extend_from_slice(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_be_bytes(),
    );

    let mut hasher = hasher.write().unwrap();
    bytes.hash(&mut *hasher);
    hasher.finish()
}

pub async fn verify(
    Query(id): Query<VerificationIDInfo>,
    State(verification_map): State<VerificationMap>,
) -> Result<JWTInfo, VerifyErrorResponse> {
    let mut verification_map = verification_map
        .write()
        .map_err(|_| VerifyErrorResponse::VerificationError)?;

    let (new_user, timestamp) = verification_map
        .remove(&id.verify_id)
        .ok_or(VerifyErrorResponse::InvalidVerificationID)?;

    let con = &mut establish_connection();

    insert_user(con, &new_user).map_err(|_| VerifyErrorResponse::UserCreationError)?;

    todo!()
}
