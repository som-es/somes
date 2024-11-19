use axum::Json;
use rand::Rng;
use rand_core::OsRng;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use somes_common_lib::{set_error_true, JWTInfo, LoginInfo};
use sqlx::{query, query_as, PgPool};

use crate::{
    email::send_otp_mail,
    hash::{self, hash_password, verify_password},
    jwt::{create_access_token, AuthError},
    model::User,
    operations::user::get_user_from_db,
    DataserviceDbConnection, PgPoolConnection, RedisConnection, EMAIL_EXPIRATION_SECONDS,
};

fn generate_otp() -> String {
    // let mut rng = rand::thread_rng();
    let mut rng = OsRng::default();

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

async fn get_user_from_mail(pg: &PgPool, stored_mail: &str) -> Option<Option<User>> {
    let maybe_user = query_as!(
        User,
        "select * from somes_user where email = $1",
        stored_mail
    )
    .fetch_optional(pg)
    .await
    .ok()?;
    match maybe_user {
        Some(user) => Some(Some(user)),
        None => {
            let hashed_email = hash_password(stored_mail).ok()?;
            query_as!(
                User,
                "select * from somes_user where email = $1",
                hashed_email
            )
            .fetch_optional(pg)
            .await
            .ok()
        }
    }
}

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:#x}", hasher.finalize())
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
    log::info!("login_info: {login_info:?}");
    let mut sign_up_error = SignUpErrorWrapper::default();

    if login_info.email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if !EMAIL_REGEX.is_match(&login_info.email) || login_info.email.len() >= 356 {
        set_error_true!(sign_up_error, invalid_email);
    }

    if sign_up_error.is_erroneous {
        return Err(SignUpErrorResponse::SignUpError(sign_up_error));
    }

    let stored_email = if login_info.hash_email.unwrap_or_default() {
        // sha256(&login_info.email)
        login_info.email.clone()
    } else {
        login_info.email.clone()
    };

    if redis_con
        .exists::<_, bool>(&stored_email)
        .await
        .unwrap_or_default()
    {
        match redis_con.get::<_, String>(&stored_email).await {
            Ok(v) => {
                let Some(password) = login_info.password else {
                    return Err(SignUpErrorResponse::WrongOtp);
                };
                let input_otp = password.trim_matches(char::is_whitespace).replace(" ", "");
                if input_otp.is_empty() {
                    return Ok(Json(JWTInfo::default()))
                }
                if verify_password(&input_otp, &v).map_err(|_| SignUpErrorResponse::Hashing)? {
                    redis_con
                        .unlink::<_, i32>(&login_info.email)
                        .await
                        .map_err(|_| SignUpErrorResponse::RedisGetKeys)?;

                    // select based on email (try with hash and without)
                    let Some(user) = get_user_from_mail(&pg, &stored_email).await else {
                        return Err(SignUpErrorResponse::PostgresConnection);
                    };

                    match user {
                        Some(user) => {
                            return create_access_token(user.id, user.email)
                                .map_err(|e| SignUpErrorResponse::AuthError(e));
                        }
                        None => {
                            let stored_email = if login_info.hash_email.unwrap_or_default() {
                                hash_password(&login_info.email)
                                    .map_err(|_| SignUpErrorResponse::Hashing)
                                    .unwrap()
                            } else {
                                login_info.email.clone()
                            };
                            // create new user
                            let id = sqlx::query!(
                                "insert into somes_user(email, is_email_hashed) values ($1, $2) returning id",
                                &stored_email, login_info.hash_email.unwrap_or_default()
                            )
                            .fetch_one(&pg)
                            .await
                            .map_err(|_| SignUpErrorResponse::PostgresConnection)?;

                            return create_access_token(id.id, stored_email)
                                .map_err(|e| SignUpErrorResponse::AuthError(e));
                        }
                    }
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

        let otp_hash = hash_password(&otp).map_err(|_| SignUpErrorResponse::Hashing)?;

        if let Err(e) = redis_con.set::<_, _, ()>(&stored_email, &otp_hash).await {
            log::error!("Failed setting email key to otp! Error: {e}");
            return Err(SignUpErrorResponse::RedisGetKeys);
        }

        if let Err(e) = redis_con
            .expire::<_, ()>(&stored_email, *EMAIL_EXPIRATION_SECONDS as i64)
            // .expire::<_, ()>(&login_info.email, 120)
            .await
        {
            log::error!("Expiration of new user entry could not be set! Error: {e}");
            redis_con
                .unlink::<_, i32>(&stored_email)
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
