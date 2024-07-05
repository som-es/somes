use axum::Json;

use somes_common_lib::SignUpInfo;

use crate::{
    email::send_verification_mail, routes::signup::action::validate_info_already_in_use_redis,
    RedisConnection, DataserviceDbConnection,
};

pub use self::{
    action::{add_new_user_to_redis, validate_signup_info},
    error::SignUpErrorResponse,
};

pub use error::*;

mod action;
mod error;

#[utoipa::path(
    post,
    path = "/signup",
    params(
        SignUpInfo
    ),
    responses(
        (status = 200, description = "Successful signup", body = [()]),
        (status = 400, description = "Invalid user data", body = [SignUpErrorWrapper]),
        (status = 500, description = "Internal server error", body = [SignUpErrorResponse])
    )
)]
pub async fn signup(
    RedisConnection(mut redis_con): RedisConnection,
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
    Json(signup_info): Json<SignUpInfo>,
) -> Result<Json<()>, SignUpErrorResponse> {
    // check if the signup info is in the temporary new user redis database
    validate_info_already_in_use_redis(&signup_info, &mut redis_con).await?;

    let int_signup_info = signup_info.clone();
    postgres_con
        .interact(move |postgres_con| validate_signup_info(postgres_con, &int_signup_info))
        .await
        .map_err(|_| SignUpErrorResponse::PostgresConnection)??;

    // checks the validity of the signup info. If this fails, the signup process is aborted.

    // if validation was successful, add a new user to the verification redis db
    let verification_id = add_new_user_to_redis(&signup_info, &mut redis_con).await?;

    // sending mail does not work currently with zimbra
    println!("http://localhost:3000/verify?id={}", verification_id);

    tokio::task::spawn_blocking(move || {
        // mails need to be encrypted!!! verify id could be grabbed
        if let Err(e) = send_verification_mail(&signup_info.email, &verification_id) {
            log::error!("Error sending verification email: {e:?}");
        }
    });

    Ok(Json(()))
}
