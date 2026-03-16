use axum::Json;
use once_cell::sync::Lazy;
use rand::Rng;
use redis::AsyncCommands;
use regex::Regex;
use somes_common_lib::{set_error_true, JWTInfo, LoginInfo};
use sqlx::{query_as, PgPool};

use crate::{
    email::send_otp_mail,
    hash::{hash_password, verify_password},
    jwt::create_access_token,
    model::User,
    routes::{SignUpErrorWrapper, UserError},
    PgPoolConnection, RedisConnection, EMAIL_EXPIRATION_SECONDS,
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

pub async fn get_user_from_mail_sqlx(
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
            let hashed_email = hash_password(stored_mail).map_err(|_| UserError::Hashing)?;
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
    PgPoolConnection(pg): PgPoolConnection,
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

    if redis_con
        .exists::<_, bool>(&stored_email)
        .await
        .unwrap_or_default()
    {
        match redis_con.get::<_, String>(&stored_email).await {
            Ok(v) => {
                let Some(password) = login_info.password else {
                    return Err(UserError::WrongOtp);
                };
                let input_otp = password.trim_matches(char::is_whitespace).replace(" ", "");
                if input_otp.is_empty() {
                    return Ok(Json(JWTInfo::default()));
                }
                if verify_password(&input_otp, &v).map_err(|_| UserError::Hashing)? {
                    redis_con.unlink::<_, i32>(&login_info.email).await?;

                    // select based on email (try with hash and without)
                    let user = get_user_from_mail_sqlx(&pg, &stored_email).await?;

                    match user {
                        Some(user) => {
                            return create_access_token(user.id, user.email, user.is_admin)
                                .map_err(|e| UserError::AuthError(e));
                        }
                        None => {
                            let stored_email = if login_info.hash_email.unwrap_or_default() {
                                hash_password(&login_info.email)
                                    .map_err(|_| UserError::Hashing)
                                    .unwrap()
                            } else {
                                login_info.email.clone()
                            };

                            let is_admin = if login_info.email == "florian.nagy@it.htl-hl.ac.at" {
                                true
                            } else {
                                false
                            };

                            // create new user
                            let id = sqlx::query!(
                                "insert into somes_user(email, is_email_hashed, is_admin) values ($1, $2, $3) returning id",
                                &stored_email, login_info.hash_email.unwrap_or_default(), is_admin
                            )
                            .fetch_one(&pg)
                            .await?;

                            return create_access_token(id.id, stored_email, false)
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
        let otp = generate_otp();

        let otp_hash = hash_password(&otp).map_err(|_| UserError::Hashing)?;

        if let Err(e) = redis_con.set::<_, _, ()>(&stored_email, &otp_hash).await {
            log::error!("Failed setting email key to otp! Error: {e}");
            return Err(UserError::RedisFailure(e));
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
                .map_err(|_| UserError::UserCreationError)?;
            return Err(UserError::UserCreationError);
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
