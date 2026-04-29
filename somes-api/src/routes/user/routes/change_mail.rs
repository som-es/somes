use axum::Json;
use once_cell::sync::Lazy;
use redis::AsyncCommands;
use regex::Regex;
use serde::{Deserialize, Serialize};
use somes_common_lib::set_error_true;
use sqlx::PgPool;

use crate::{
    hash::verify_password,
    jwt::{create_access_token, Claims},
    model::User,
    routes::{send_otp, SignUpErrorWrapper, UserError},
    PgPoolConnection, RedisConnection,
};

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
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(body): Json<ChangeMailBody>,
) -> Result<Json<ChangeMailResponse>, UserError> {
    let mut sign_up_error = SignUpErrorWrapper::default();

    if body.new_email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if !EMAIL_REGEX.is_match(&body.new_email) || body.new_email.len() >= 356 {
        set_error_true!(sign_up_error, invalid_email);
    }

    if sign_up_error.is_erroneous {
        return Err(UserError::SignUpError(sign_up_error));
    }

    let stored_email = format!("change_mail/{}", body.new_email.clone());

    if claims.is_anonymised {
        let skip_otp =
            verify_password(&body.new_email, &claims.sub).map_err(|_| UserError::Hashing)?;

        if skip_otp {
            let jwt_info = change_to_clear_text_email(pg, claims, &body.new_email).await?;
            return Ok(Json(ChangeMailResponse {
                success: true,
                message: "E-Mail-Adresse erfolgreich entanonymisiert.".to_string(),
                requires_otp: false,
                access_token: Some(jwt_info.access_token),
            }));
        }
    }

    if redis_con
        .exists::<_, bool>(&stored_email)
        .await
        .unwrap_or_default()
    {
        return Ok(Json(ChangeMailResponse {
            success: true,
            message: "An deine E-Mail-Adresse wurde bereits ein One-Time Passwort gesendet."
                .to_string(),
            requires_otp: true,
            access_token: None,
        }));
    } else {
        send_otp(&mut redis_con, &body.new_email, &stored_email).await?;
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
    if body.new_email.is_empty() || body.otp.is_empty() {
        return Err(UserError::WrongOtp);
    }

    let input_otp = body.otp.trim_matches(char::is_whitespace).replace(" ", "");

    let email_key = format!("change_mail/{}", body.new_email);

    let stored_hash = match redis_con.get::<_, String>(&email_key).await {
        Ok(v) => v,
        Err(_) => {
            log::warn!("No Redis entry found for email: {}", body.new_email);
            return Err(UserError::WrongOtp);
        }
    };

    let is_valid = verify_password(&input_otp, &stored_hash).map_err(|_| UserError::Hashing)?;

    if !is_valid {
        log::warn!("Invalid OTP for email: {}", body.new_email);
        return Err(UserError::WrongOtp);
    }

    redis_con.unlink::<_, i32>(&email_key).await?;

    let jwt_info = change_to_clear_text_email(pg, claims, &body.new_email).await?;

    Ok(Json(ChangeMailResponse {
        success: true,
        message: "E-Mail-Adresse erfolgreich geändert.".to_string(),
        requires_otp: false,
        access_token: Some(jwt_info.access_token),
    }))
}

async fn change_to_clear_text_email(
    pg: sqlx::Pool<sqlx::Postgres>,
    claims: crate::jwt::ClaimsGen<i32>,
    new_email: &str,
) -> Result<somes_common_lib::JWTInfo, UserError> {
    let current_user = get_current_user_from_sqlx(&pg, claims.id).await?;
    let user = match current_user {
        Some(user) => user,
        None => return Err(UserError::UserCreationError),
    };
    sqlx::query!(
        "update somes_user set email = $1, is_email_hashed = false where id = $2",
        new_email,
        user.id
    )
    .execute(&pg)
    .await
    .map_err(|e| {
        log::error!("Failed to update email for user {}: {:?}", user.id, e);
        UserError::UserCreationError
    })?;
    let Json(jwt_info) = create_access_token(
        user.id,
        new_email.to_string(),
        user.is_admin,
        false,
    )
    .map_err(UserError::AuthError)?;
    Ok(jwt_info)
}
