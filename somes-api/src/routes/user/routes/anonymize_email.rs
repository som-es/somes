use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    hash::hash_password,
    jwt::{create_access_token, Claims},
    model::User,
    routes::UserError,
    PgPoolConnection,
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
) -> Result<Json<AnonymizeEmailResponse>, UserError> {
    let current_user = sqlx::query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await?;

    if current_user.is_email_hashed {
        return Err(UserError::AlreadyAnonymised);
    }

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
    let new_token = create_access_token(claims.id, hashed_email, claims.is_admin, true)
        .map_err(|_| UserError::UserCreationError)?;

    Ok(Json(AnonymizeEmailResponse {
        success: true,
        message: "E-Mail wurde erfolgreich anonymisiert.".to_string(),
        requires_otp: false,
        access_token: Some(new_token.access_token.clone()),
    }))
}
