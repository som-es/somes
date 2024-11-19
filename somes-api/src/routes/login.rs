use axum::Json;
use rand::Rng;
use redis::AsyncCommands;
use somes_common_lib::{set_error_true, JWTInfo, LoginInfo};

use crate::{
    email::send_otp_mail,
    hash,
    jwt::{create_access_token, AuthError},
    operations::user::get_user_from_db,
    DataserviceDbConnection, PgPoolConnection, RedisConnection, EMAIL_EXPIRATION_SECONDS,
};

fn generate_otp() -> String {
    let mut rng = rand::thread_rng();

    (0..9)
        .map(|_| {
            if rng.gen_range(0f32..1f32) > 0.2 {
                rng.gen_range('A'..='Z')
            } else {
                rng.gen_range('0'..='9')
            }
        })
        .collect()
}

use super::{action::EMAIL_REGEX, SignUpErrorResponse, SignUpErrorWrapper};

#[utoipa::path(
    post,
    path = "/login",
    params(
        LoginInfo
    ),
    responses(
        (status = 200, description = "Successful login", body = [JWTInfo]),
        (status = 401, description = "Invalid credentials", body = [AuthError]),
        (status = 400, description = "Invalid request", body = [AuthError]),
        (status = 500, description = "Internal server error", body = [AuthError])
    )
)]
pub async fn login(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<JWTInfo>, SignUpErrorResponse> {
    let mut sign_up_error = SignUpErrorWrapper::default();
    return Err(SignUpErrorResponse::PostgresConnection);

    if login_info.email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if !EMAIL_REGEX.is_match(&login_info.email) {
        set_error_true!(sign_up_error, invalid_email);
    }

    if redis_con
        .exists::<_, bool>(&login_info.email)
        .await
        .unwrap_or_default()
    {
        match redis_con.get::<_, String>(&login_info.email).await {
            Ok(v) => {
                let input_otp = login_info.password.trim_matches(char::is_whitespace).replace(" ", "");
                if v == input_otp {
                    redis_con
                        .unlink::<_, i32>(&login_info.email)
                        .await
                        .map_err(|_| SignUpErrorResponse::RedisGetKeys)?;
                } else {
                    return Err(SignUpErrorResponse::WrongOtp);
                }
            }
            Err(e) => {
                log::error!("Failed getting email key! Error: {e}");
                return Err(SignUpErrorResponse::RedisGetKeys);
            }
        }
    } else {
        let otp = generate_otp();
        if let Err(e) = redis_con.set::<_, _, ()>(&login_info.email, &otp).await {
            log::error!("Failed setting email key to otp! Error: {e}");
            return Err(SignUpErrorResponse::RedisGetKeys);
        }

        if let Err(e) = redis_con
            .expire::<_, ()>(&login_info.email, *EMAIL_EXPIRATION_SECONDS as i64)
            // .expire::<_, ()>(&login_info.email, 120)
            .await
        {
            log::error!("Expiration of new user entry could not be set! Error: {e}");
            redis_con
                .unlink::<_, i32>(&login_info.email)
                .await
                .map_err(|_| SignUpErrorResponse::UserCreationError)?;
            return Err(SignUpErrorResponse::UserCreationError);
        }

        tokio::task::spawn_blocking(move || {
            // mails need to be encrypted!!! verify id could be grabbed
            if let Err(e) = send_otp_mail(&login_info.email, &otp) {
                log::error!("Error sending verification email: {e:?}");
            }
        });
    }

    // check redis

    // mitigate brute force attacks
    // Start showing CAPTCHAs after three incorrect attempts from an IP
    // After an incorrect attempt, block all new login requests to your server from that IP for a period of time. Increment this on every failed attempt.
    // Keep a log and note spikes of activity. If someone is trying to bruteforce, you ought to make note of that and counter it.

    // let con = &mut establish_connection();

    if sign_up_error.is_erroneous {
        return Err(SignUpErrorResponse::SignUpError(sign_up_error));
    }

    Ok(Json(JWTInfo::default()))

    // let user = con
    //     .interact(move |con| {
    //         get_user_from_db(
    //             con,
    //             &login_info.email,
    //             &login_info.email,
    //         )
    //         .ok_or(AuthError::WrongCredentials)
    //     })
    //     .await
    //     .map_err(|_| AuthError::WrongCredentials)??;

    // if !hash::verify_password(&login_info.password, &user.password_hash)
    //     .map_err(|_| AuthError::WrongCredentials)?
    // {
    //     return Err(AuthError::WrongCredentials);
    // }

    // create_access_token(user.id, user.username)
}
