use axum::Json;
use somes_common_lib::SignUpInfo;

use crate::{db::establish_connection, RedisConnection};

pub use self::{
    action::{add_new_user_to_redis, validate_signup_info},
    error::SignUpErrorResponse,
};

mod action;
mod error;

pub async fn signup(
    RedisConnection(mut conn): RedisConnection,
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<()>, SignUpErrorResponse> {
    let mut con = establish_connection();

    // checks  the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &signup_info)?;

    // if validation was successful, add a new user to the verification redis db
    let _id = add_new_user_to_redis(signup_info, &mut conn).await;
    println!("id: {_id:?}");
    Ok(Json(()))
}
