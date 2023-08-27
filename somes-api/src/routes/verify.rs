use redis::AsyncCommands;
use sha3::{Digest, Sha3_256};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{extract::Query, Json};
use somes_common_lib::{JWTInfo, SignUpInfo, VerificationIDInfo};
use uuid::Uuid;

use crate::{
    jwt::create_access_token, model::NewUser, operations::user::insert_user, RedisConnection,
    SomesDbConnection,
};

// use self::error::VerifyErrorResponse;

pub use error::VerifyErrorResponse;

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

#[utoipa::path(
    get,
    path = "/verify",
    params(
        VerificationIDInfo
    ),
    responses(
        (status = 200, description = "Successful verification of a new user account-> log user in", body = [JWTInfo]),
        (status = 400, description = "Internal server error", body = [VerifyErrorResponse])
    )
)]
pub async fn verify(
    Query(id): Query<VerificationIDInfo>,
    RedisConnection(redis_con): RedisConnection,
    SomesDbConnection(con): SomesDbConnection,
    //State(verification_map): State<VerificationMap>,
) -> Result<Json<JWTInfo>, VerifyErrorResponse> {
    //let new_user = remove_from_verify_map(verification_map, &id)?;
    let new_user = remove_user_from_redis(redis_con, &id).await?;

    // let con = &mut establish_connection();
    let username = new_user.username.clone();
    let id = con
        .interact(move |con| {
            insert_user(con, &new_user).map_err(|_| VerifyErrorResponse::UserCreationError)
        })
        .await
        .map_err(|_| VerifyErrorResponse::UserCreationError)??;

    // let id = insert_user(con, &new_user).map_err(|_| VerifyErrorResponse::UserCreationError)?;

    create_access_token(id, username).map_err(VerifyErrorResponse::Auth)
}

pub async fn remove_user_from_redis(
    mut redis_con: redis::aio::Connection,
    id: &VerificationIDInfo,
) -> Result<NewUser, VerifyErrorResponse> {
    let new_user = redis_con
        .get::<_, NewUser>(&id.id)
        .await
        .map_err(|_| VerifyErrorResponse::InvalidVerificationID)?;

    redis_con
        .unlink::<_, u32>(&id.id)
        .await
        .map_err(|_| VerifyErrorResponse::InvalidVerificationID)?;

    Ok(new_user)
}

#[cfg(test)]
mod tests {
    use diesel::Connection;
    use somes_common_lib::{SignUpInfo, VerificationIDInfo};

    use crate::{
        establish_connection,
        routes::{add_new_user_to_redis, remove_user_from_redis, validate_signup_info},
        REDIS_DB,
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

    #[tokio::test]
    async fn test_verify_process() {
        let client = redis::Client::open(REDIS_DB).unwrap();
        let mut redis_con = client.get_async_connection().await.unwrap();

        //let verification_map = VerificationMap::default();

        let signup_info = SignUpInfo {
            email: "test1@test.at".to_string(),
            username: "test_name".to_string(),
            password: "suPERs%icCHER123asdr".to_string(),
        };

        let con = &mut establish_connection();
        con.test_transaction::<_, (), _>(|con| {
            validate_signup_info(con, &signup_info).unwrap();
            Ok(())
        });

        let id = add_new_user_to_redis(&signup_info, &mut redis_con)
            .await
            .unwrap();

        remove_user_from_redis(redis_con, &VerificationIDInfo { id })
            .await
            .unwrap();

        // println!("veri: {verification_map:?}");
    }
}
