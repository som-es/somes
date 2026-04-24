use axum::Json;
use once_cell::sync::Lazy;
use rand::Rng;
use redis::AsyncCommands;
use regex::Regex;
use serde::{Deserialize, Serialize};
use somes_common_lib::set_error_true;
use sqlx::PgPool;

use crate::{
    email::send_otp_mail,
    hash::{hash_password, verify_password},
    jwt::{create_access_token, Claims},
    model::User,
    routes::{SignUpErrorWrapper, UserError},
    PgPoolConnection, RedisConnection, EMAIL_EXPIRATION_SECONDS,
};

fn generate_otp() -> String {
    let mut rng = rand::rng();
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

pub static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^@]+@[^@]+\.[^@]+").expect("Invalid email regex"));

pub async fn get_current_user_from_sqlx(
    pg: &PgPool,
    user_id: i32,
) -> Result<Option<User>, UserError> {
    let maybe_user = sqlx::query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        user_id
    )
    .fetch_optional(pg)
    .await?;
    Ok(maybe_user)
}

#[derive(Deserialize)]
pub struct ChangeMailBody {
    pub new_email: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailChangeBody {
    pub new_email: String,
    pub otp: String,
}

#[derive(Serialize)]
pub struct ChangeMailResponse {
    pub success: bool,
    pub message: String,
    pub requires_otp: bool,
    pub access_token: Option<String>,
}

pub async fn change_mail(
    RedisConnection(mut redis_con): RedisConnection,
    Json(body): Json<ChangeMailBody>,
) -> Result<Json<ChangeMailResponse>, UserError> {
    println!("DEBUG: change_mail called with email: {}", body.new_email);

    let mut sign_up_error = SignUpErrorWrapper::default();

    if body.new_email.is_empty() {
        println!("DEBUG: Email is empty");
        set_error_true!(sign_up_error, missing_email);
    }

    if !EMAIL_REGEX.is_match(&body.new_email) || body.new_email.len() >= 356 {
        println!(
            "DEBUG: Email validation failed - regex match: {}, length: {}",
            EMAIL_REGEX.is_match(&body.new_email),
            body.new_email.len()
        );
        set_error_true!(sign_up_error, invalid_email);
    }

    if sign_up_error.is_erroneous {
        println!("DEBUG: Returning SignUpError");
        return Err(UserError::SignUpError(sign_up_error));
    }

    let stored_email = body.new_email.clone();

    if redis_con
        .exists::<_, bool>(&stored_email)
        .await
        .unwrap_or_default()
    {
        return Err(UserError::WrongOtp);
    } else {
        let otp = generate_otp();
        println!("OTP: {}", otp);
        let otp_hash = hash_password(&otp).map_err(|_| UserError::Hashing)?;

        if let Err(e) = redis_con.set::<_, _, ()>(&stored_email, &otp_hash).await {
            log::error!("Failed setting email key to otp! Error: {e}");
            return Err(UserError::RedisFailure(e));
        }

        if let Err(e) = redis_con
            .expire::<_, ()>(&stored_email, *EMAIL_EXPIRATION_SECONDS as i64)
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
            if let Err(e) = send_otp_mail(&body.new_email, &otp) {
                log::error!("Error sending verification email: {e:?}");
            }
        });
    }

    Ok(Json(ChangeMailResponse {
        success: true,
        message: "An deine E-Mail-Adresse wurde ein One-Time Passwort gesendet.".to_string(),
        requires_otp: true,
        access_token: None,
    }))
}
pub async fn verify_email_change(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(body): Json<VerifyEmailChangeBody>,
) -> Result<Json<ChangeMailResponse>, UserError> {
    println!("DEBUG: verify_email_change START");
    println!("DEBUG: user_id = {}", claims.id);
    println!("DEBUG: email = {}", body.new_email);
    println!("DEBUG: otp = {}", body.otp);

    if body.new_email.is_empty() || body.otp.is_empty() {
        return Err(UserError::WrongOtp);
    }

    let input_otp = body.otp.trim_matches(char::is_whitespace).replace(" ", "");

    let stored_hash = match redis_con.get::<_, String>(&body.new_email).await {
        Ok(v) => v,
        Err(_) => {
            println!("DEBUG: no redis entry found");
            return Err(UserError::WrongOtp);
        }
    };

    let is_valid = verify_password(&input_otp, &stored_hash).map_err(|_| UserError::Hashing)?;

    if !is_valid {
        println!("DEBUG: otp invalid");
        return Err(UserError::WrongOtp);
    }

    redis_con.unlink::<_, i32>(&body.new_email).await?;

    let current_user = get_current_user_from_sqlx(&pg, claims.id).await?;

    let user = match current_user {
        Some(user) => user,
        None => return Err(UserError::UserCreationError),
    };

    sqlx::query!(
        "update somes_user set email = $1, is_email_hashed = false where id = $2",
        &body.new_email,
        user.id
    )
    .execute(&pg)
    .await
    .map_err(|e| {
        println!("SQL ERROR: {:?}", e);
        UserError::UserCreationError
    })?;

    println!("DEBUG: email updated");

   let Json(jwt_info) = create_access_token(user.id, body.new_email.clone(), user.is_admin)
    .map_err(UserError::AuthError)?;

    println!("DEBUG: email updated + jwt created");

    Ok(Json(ChangeMailResponse {
        success: true,
        message: "E-Mail-Adresse erfolgreich geändert.".to_string(),
        requires_otp: false,
        access_token: Some(jwt_info.access_token),
    }))
}
