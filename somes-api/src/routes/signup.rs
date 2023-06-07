use axum::Json;

use somes_common_lib::SignUpInfo;

use crate::{
    db::establish_connection, email::send_mail,
    routes::signup::action::validate_info_already_in_use_redis, RedisConnection,
};

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
    let verification_id = add_new_user_to_redis(&signup_info, &mut redis_con).await?;

    tokio::task::spawn_blocking(move || {
        // mails need to be encrypted!!! verify id could be grabbed
        if let Err(e) = send_mail(&signup_info.email, &verification_id) {
            log::warn!("Error sending verification email: {e:?}");
        }
    });

    Ok(Json(()))
}
