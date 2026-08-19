use axum::Json;
use once_cell::sync::Lazy;
use rand::Rng;
use redis::AsyncCommands;
use regex::Regex;
use somes_common_lib::{JWTInfo, LoginInfo, set_error_true};
use sqlx::{PgPool, query_as};

use crate::{
    AtPgPoolConnection, EMAIL_EXPIRATION_SECONDS, RedisConnection,
    email::send_otp_mail,
    hash::{hash_password, verify_password},
    jwt::create_access_token,
    model::User,
    routes::{SignUpErrorWrapper, UserError},
};

fn generate_otp() -> String {
    let mut rng = rand::rng();
    // let mut rng = OsRng::default();

    (0..9)
        .map(|_| {
            if rng.random_range(0f32..1f32) > 0.2 {
                rng.random_range('A'..='Z')
            } else {
                rng.random_range('0'..='9')
            }
        })
        .collect()
}

pub async fn get_user_from_mail_or_hash_sqlx(
    pg: &PgPool,
    stored_mail: &str,
) -> Result<Option<User>, UserError> {
    let maybe_user = query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where email = $1",
        stored_mail
    )
    .fetch_optional(pg)
    .await?;
    match maybe_user {
        Some(user) => Ok(Some(user)),
        None => {
            let hashed_email = hash_password(stored_mail, false).map_err(|_| UserError::Hashing)?;
            Ok(query_as!(
                User,
                "select id, email, is_email_hashed, is_admin from somes_user where email = $1",
                hashed_email
            )
            .fetch_optional(pg)
            .await?)
        }
    }
}
pub async fn send_otp(
    redis_con: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    email: &str,
    stored_email: &str,
) -> Result<(), UserError> {
    let otp = generate_otp();

    let otp_hash = hash_password(&otp, true).map_err(|_| UserError::Hashing)?;

    if let Err(e) = redis_con.set::<_, _, ()>(stored_email, &otp_hash).await {
        log::error!("Failed setting email key to otp! Error: {e}");
        return Err(UserError::RedisFailure(e));
    }

    if let Err(e) = redis_con
        .expire::<_, ()>(stored_email, *EMAIL_EXPIRATION_SECONDS as i64)
        .await
    {
        log::error!("Expiration of otp could not be set! Error: {e}");

        redis_con
            .unlink::<_, i32>(stored_email)
            .await
            .map_err(|_| UserError::UserCreationError)?;

        return Err(UserError::UserCreationError);
    }

    let email = email.to_string();
    tokio::task::spawn_blocking(move || {
        if let Err(e) = send_otp_mail(&email, &otp) {
            log::error!("Error sending verification email: {e:?}");
        }
    });

    Ok(())
}

pub static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^@]+@[^@]+\.[^@]+").expect("Invalid email regex"));

#[utoipa::path(
    post,
    path = "/login",
    params(
        LoginInfo
    ),
    responses(
        (status = 200, description = "Successful login", body = [JWTInfo]),
        // (status = 401, description = "Invalid credentials", body = [UserError]),
        // (status = 400, description = "Invalid request", body = [UserError]),
        // (status = 500, description = "Internal server error", body = [UserError])
    )
)]
pub async fn login(
    RedisConnection(mut redis_con): RedisConnection,
    AtPgPoolConnection(pg): AtPgPoolConnection,
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<JWTInfo>, UserError> {
    let mut sign_up_error = SignUpErrorWrapper::default();

    if login_info.email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if !EMAIL_REGEX.is_match(&login_info.email) || login_info.email.len() >= 356 {
        set_error_true!(sign_up_error, invalid_email);
    }

    if sign_up_error.is_erroneous {
        return Err(UserError::SignUpError(sign_up_error));
    }

    let stored_email = if login_info.hash_email.unwrap_or_default() {
        // sha256(&login_info.email)
        login_info.email.clone()
    } else {
        login_info.email.clone()
    };

    let key = format!("login/{stored_email}");

    if redis_con.exists::<_, bool>(&key).await.unwrap_or_default() {
        match redis_con.get::<_, String>(&key).await {
            Ok(v) => {
                let Some(password) = login_info.password else {
                    return Err(UserError::WrongOtp);
                };
                let input_otp = password.trim_matches(char::is_whitespace).replace(" ", "");
                if input_otp.is_empty() {
                    return Ok(Json(JWTInfo::default()));
                }
                if verify_password(&input_otp, &v).map_err(|_| UserError::Hashing)? {
                    redis_con.unlink::<_, i32>(&key).await?;

                    // select based on email (try with hash and without)
                    let user = get_user_from_mail_or_hash_sqlx(&pg, &stored_email).await?;

                    match user {
                        Some(user) => {
                            return create_access_token(
                                user.id,
                                user.email,
                                user.is_admin,
                                user.is_email_hashed,
                            )
                            .map_err(|e| UserError::AuthError(e));
                        }
                        None => {
                            let stored_email = if login_info.hash_email.unwrap_or_default() {
                                hash_password(&login_info.email, false)
                                    .map_err(|_| UserError::Hashing)
                                    .unwrap()
                            } else {
                                login_info.email.clone()
                            };

                            let id = sqlx::query!(
                                "insert into somes_user(email, is_email_hashed, is_admin) values ($1, $2, $3) returning id",
                                &stored_email, login_info.hash_email.unwrap_or_default(), false
                            )
                            .fetch_one(&pg)
                            .await?;

                            return create_access_token(
                                id.id,
                                stored_email,
                                false,
                                login_info.hash_email.unwrap_or_default(),
                            )
                            .map_err(|e| UserError::AuthError(e));
                        }
                    }
                } else {
                    return Err(UserError::WrongOtp);
                }
            }
            Err(e) => {
                log::error!("Failed getting email key! Error: {e}");
                return Err(UserError::RedisFailure(e));
            }
        }
    } else {
        send_otp(&mut redis_con, &login_info.email, &key).await?;
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
