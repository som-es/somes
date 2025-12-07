use std::ops::{Deref, DerefMut};

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use somes_common_lib::errors::SignUpError;
use utoipa::ToSchema;

use crate::AuthError;

#[derive(Debug, ToSchema)]
pub enum UserErrorResponse {
    RedisGetKeys,
    PostgresConnection,
    InvalidUser,
    InteractionFailed,
    VerificationEmailSendingError,
    UserCreationError,
    WrongOtp,
    Hashing,
    AuthError(AuthError),
    SignUpError(SignUpErrorWrapper),
}

impl IntoResponse for UserErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            UserErrorResponse::RedisGetKeys => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred. Redis active user matching failed!",
            ),
            UserErrorResponse::PostgresConnection => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred. Postgres connection failed!",
            ),
            UserErrorResponse::InvalidUser => (
                StatusCode::BAD_REQUEST,
                "Invalid user id provided. User does not exist.",
            ),
            UserErrorResponse::InteractionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interaction with database failed.",
            ),
            UserErrorResponse::VerificationEmailSendingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred. Sending verification email was unseccessful!",
            ),
            UserErrorResponse::UserCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred. Creating user was unsuccessful!",
            ),
            UserErrorResponse::Hashing => (StatusCode::INTERNAL_SERVER_ERROR, "Hashing error"),
            UserErrorResponse::SignUpError(signup_error) => {
                return (StatusCode::BAD_REQUEST, Json(signup_error.deref())).into_response()
            }
            UserErrorResponse::WrongOtp => (StatusCode::BAD_REQUEST, "Wrong OTP"),
            UserErrorResponse::AuthError(ae) => return ae.into_response(),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

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
