use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{hash::hash_password, jwt::Claims, model::User, routes::UserError, PgPoolConnection};

#[derive(Deserialize, Debug)]
pub struct AnonymizeEmailBody {
    pub anonymize: bool,
}

#[derive(Serialize, Debug)]
pub struct AnonymizeEmailResponse {
    pub success: bool,
    pub message: String,
}

pub async fn anonymize_email(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    Json(body): Json<AnonymizeEmailBody>,
) -> Result<Json<AnonymizeEmailResponse>, UserError> {
    println!(
        "Anonymize email request: {:?} {} - {}",
        body, claims.id, claims.sub
    );

    let current_user = sqlx::query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await?;

    if body.anonymize && !current_user.is_email_hashed {
        let hashed_email = hash_password(&claims.sub).map_err(|_| UserError::Hashing)?;

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

        Ok(Json(AnonymizeEmailResponse {
            success: true,
            message: "E-Mail wurde erfolgreich anonymisiert.".to_string(),
        }))
    } else if !body.anonymize && current_user.is_email_hashed {
        sqlx::query!(
            "update somes_user set email = $1, is_email_hashed = false where id = $2",
            claims.sub,
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

        Ok(Json(AnonymizeEmailResponse {
            success: true,
            message: "E-Mail-Anonymisierung wurde aufgehoben.".to_string(),
        }))
    } else {
        Ok(Json(AnonymizeEmailResponse {
            success: true,
            message: "Keine Änderung erforderlich.".to_string(),
        }))
    }
}
