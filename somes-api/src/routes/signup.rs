use axum::Json;
use diesel::connection;
use somes_common_lib::SignUpInfo;

use crate::{db::establish_connection, RedisConnection, routes::signup::action::validate_info_already_in_use_redis};

pub use self::{
    action::{add_new_user_to_redis, validate_signup_info},
    error::SignUpErrorResponse,
};

mod action;
mod error;

pub async fn signup(
    RedisConnection(mut redis_con): RedisConnection,
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<()>, SignUpErrorResponse> {
    let mut con = establish_connection();

    // check if the signup info is in the temporary new user redis database
    validate_info_already_in_use_redis(&signup_info, &mut redis_con).await?;

    // checks the validity of the signup info. If this fails, the signup process is aborted.
    validate_signup_info(&mut con, &signup_info)?;

    // if validation was successful, add a new user to the verification redis db
    let _id = add_new_user_to_redis(signup_info, &mut redis_con).await;

    // send mail?
    println!("id: {_id:?}");
    Ok(Json(()))
}
