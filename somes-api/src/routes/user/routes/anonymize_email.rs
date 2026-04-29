use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    hash::hash_password,
    jwt::{create_access_token, Claims},
    model::User,
    routes::{send_otp, UserError},
    PgPoolConnection, RedisConnection,
};

#[derive(Deserialize, Debug)]
pub struct AnonymizeEmailBody {
    pub anonymize: bool,
    pub email: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AnonymizeEmailResponse {
    pub success: bool,
    pub message: String,
    pub requires_otp: bool,
    pub access_token: Option<String>,
}

pub async fn anonymize_email(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Json(body): Json<AnonymizeEmailBody>,
) -> Result<Json<AnonymizeEmailResponse>, UserError> {


    let current_user = sqlx::query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await?;

    if body.anonymize && !current_user.is_email_hashed {
        let hashed_email = hash_password(&claims.sub, false).map_err(|_| UserError::Hashing)?;

        sqlx::query!(
            "update somes_user set email = $1, is_email_hashed = true where id = $2",
            hashed_email,
            current_user.id
        )
        .execute(&pg)
        .await
        .map_err(|e| {
            log::error!(
                "Failed to anonymize email for user {}: {:?}",
                current_user.id,
                e
            );
            UserError::UserCreationError
        })?;

        // Create new JWT token with hashed email
        let new_token = create_access_token(claims.id, hashed_email, claims.is_admin)
            .map_err(|_| UserError::UserCreationError)?;

        Ok(Json(AnonymizeEmailResponse {
            success: true,
            message: "E-Mail wurde erfolgreich anonymisiert.".to_string(),
            requires_otp: false,
            access_token: Some(new_token.access_token.clone()),
        }))
    } else if !body.anonymize && current_user.is_email_hashed {
        let email = body.email.as_deref().unwrap_or("");
        let hashed_input_email = hash_password(email, false).map_err(|_| UserError::Hashing)?;

        if current_user.email == hashed_input_email {
            sqlx::query!(
                "update somes_user set email = $1, is_email_hashed = false where id = $2",
                email,
                current_user.id
            )
            .execute(&pg)
            .await
            .map_err(|e| {
                log::error!(
                    "Failed to de-anonymize email for user {}: {:?}",
                    current_user.id,
                    e
                );
                UserError::UserCreationError
            })?;

            let new_token = create_access_token(claims.id, email.to_string(), claims.is_admin)
                .map_err(|_| UserError::UserCreationError)?;

            Ok(Json(AnonymizeEmailResponse {
                success: true,
                message: "E-Mail-Anonymisierung wurde aufgehoben.".to_string(),
                requires_otp: false,
                access_token: Some(new_token.access_token.clone()),
            }))
        } else {
            let redis_key = email;

            send_otp(&mut redis_con, &email, &redis_key).await?;

            return Ok(Json(AnonymizeEmailResponse {
                success: true,
                message: "OTP wurde an die E-Mail-Adresse gesendet.".into(),
                requires_otp: true,
                access_token: None,
            }));
        }
    } else {
        Ok(Json(AnonymizeEmailResponse {
            success: true,
            message: "Keine Änderung erforderlich.".to_string(),
            requires_otp: false,
            access_token: None,
        }))
    }
}
