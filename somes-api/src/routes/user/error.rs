use std::ops::{Deref, DerefMut};

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use somes_common_lib::errors::SignUpError;
use thiserror::Error;
use utoipa::ToSchema;

use crate::{AuthError, ErrorInfo};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Database failure: {0}")]
    SqlFailure(#[from] sqlx::Error),
    #[error("Redis failure: {0}")]
    RedisFailure(#[from] redis::RedisError),
    #[error("Meilisearch failure: {0}")]
    MeilisearchFailure(#[from] meilisearch_sdk::errors::Error),
    #[error("internal server error")]
    InternalServerError,
    #[error("invalid user")]
    InvalidUser,
    #[error("interaction failed")]
    InteractionFailed,
    #[error("verification email sending error")]
    VerificationEmailSendingError,
    #[error("user creation error")]
    UserCreationError,
    #[error("wrong OTP")]
    WrongOtp,
    #[error("hashing error")]
    Hashing,
    #[error("authentication error")]
    AuthError(AuthError),
    #[error("sign up error")]
    SignUpError(SignUpErrorWrapper),
    #[error("{1}")]
    Custom(StatusCode, String),
    #[error("missing email from OAuth provider")]
    MissingEmail,
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            UserError::SqlFailure(_e) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            UserError::RedisFailure(_e) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            UserError::MeilisearchFailure(_e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            UserError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            UserError::InvalidUser => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::InteractionFailed => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::VerificationEmailSendingError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            UserError::UserCreationError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            UserError::Hashing => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            UserError::WrongOtp => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::SignUpError(signup_error) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorInfo {
                        error: format!("{:?}", signup_error.sign_up_error),
                        error_type: "SignUpError",
                        field: format!("{:?}", self),
                        meta: serde_json::to_value(&signup_error.sign_up_error).ok(),
                    }),
                )
                    .into_response();
            }
            UserError::AuthError(ae) => return ae.into_response(),
            UserError::Custom(code, msg) => (*code, msg.clone()),
            UserError::MissingEmail => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "UserError",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}

#[derive(Debug, Default, ToSchema)]
pub struct SignUpErrorWrapper {
    pub sign_up_error: SignUpError,
}

impl Deref for SignUpErrorWrapper {
    type Target = SignUpError;

    fn deref(&self) -> &Self::Target {
        &self.sign_up_error
    }
}

impl DerefMut for SignUpErrorWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sign_up_error
    }
}
