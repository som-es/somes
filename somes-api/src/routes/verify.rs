
use sha3::{Digest, Sha3_256};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Query, State},
    Json,
};
use somes_common_lib::{JWTInfo, SignUpInfo, VerificationIDInfo};
use uuid::Uuid;

use crate::{
    establish_connection,
    jwt::create_access_token,
    model::NewUser,
    operations::user::insert_user,
    server::{VerificationMap, RedisConn},
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
    State(redis_con): State<RedisConn>,
    State(verification_map): State<VerificationMap>,
) -> Result<Json<JWTInfo>, VerifyErrorResponse> {
    let new_user = remove_from_verify_map(verification_map, &id)?;
    let con = &mut establish_connection();

    insert_user(con, &new_user).map_err(|_| VerifyErrorResponse::UserCreationError)?;

    create_access_token(new_user.username)
        .map_err(VerifyErrorResponse::Auth)
}

pub fn remove_from_verify_map(
    verification_map: VerificationMap,
    id: &VerificationIDInfo,
) -> Result<NewUser, VerifyErrorResponse> {
    let mut verification_map = verification_map
        .write()
        .map_err(|_| VerifyErrorResponse::VerificationError)?;

    let (new_user, _timestamp) = verification_map
        .remove(&id.verify_id)
        .ok_or(VerifyErrorResponse::InvalidVerificationID)?;

    Ok(new_user)
}

#[cfg(test)]
mod tests {
    use somes_common_lib::{SignUpInfo, VerificationIDInfo};

    use crate::{
        id,
        routes::{
            add_new_user_to_verification_map, validate_signup_info, verify::remove_from_verify_map,
        },
        server::VerificationMap,
        test_db,
    };

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

    #[test]
    fn test_verify_process() {
        let db = test_db::create_handle(id!());
        let mut con = db.establish_connection();
        let verification_map = VerificationMap::default();

        let signup_info = SignUpInfo {
            email: "test1@test.at".to_string(),
            username: "test_name".to_string(),
            password: "suPERs%icCHER123asdr".to_string(),
        };

        validate_signup_info(&mut con, &signup_info, verification_map.clone()).unwrap();
        let verify_id =
            add_new_user_to_verification_map(signup_info, verification_map.clone()).unwrap();

        remove_from_verify_map(verification_map.clone(), &VerificationIDInfo { verify_id })
            .unwrap();

        println!("veri: {verification_map:?}");
    }
}
