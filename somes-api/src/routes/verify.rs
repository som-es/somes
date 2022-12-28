use sha3::{Digest, Sha3_256};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Query, State};
use somes_common_lib::{JWTInfo, SignUpInfo, VerificationIDInfo, time::{timestamp_secs, is_verify_id_valid}};
use uuid::Uuid;

use crate::{
    establish_connection,
    operations::user::insert_user,
    server::VerificationMap
};

use self::error::VerifyErrorResponse;

mod error;

pub fn create_verification_id(signup_info: &SignUpInfo) -> String {
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

    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);
    
    base16ct::lower::encode_string(&hasher.finalize())
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


    if !is_verify_id_valid(timestamp, timestamp_secs()) {
        return Err(VerifyErrorResponse::InvalidVerificationID)
    }
    

    let con = &mut establish_connection();

    insert_user(con, &new_user).map_err(|_| VerifyErrorResponse::UserCreationError)?;

    todo!()
}


#[cfg(test)]
mod tests {
    use somes_common_lib::SignUpInfo;

    use super::create_verification_id;

    #[test]
    fn test_create_verification_id() {
        let signup_info = SignUpInfo {
            email: "test1@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        create_verification_id(&signup_info);
    }
}