use std::ops::{Deref, DerefMut};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use somes_common_lib::errors::SignUpError;
use utoipa::ToSchema;

use crate::AuthError;

#[derive(Debug, ToSchema)]
pub enum LoginErrorResponse {
    PostgresConnection,
    RedisGetKeys,
    VerificationEmailSendingError,
    UserCreationError,
    WrongOtp,
    Hashing,
    AuthError(AuthError),
    SignUpError(SignUpErrorWrapper),
}

impl IntoResponse for LoginErrorResponse {
    fn into_response(self) -> Response {
        match self {
            LoginErrorResponse::PostgresConnection => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Postgres connection failed!"})),
            ).into_response(),
            LoginErrorResponse::RedisGetKeys => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Redis active user matching failed!"})),
            ).into_response(),
            LoginErrorResponse::VerificationEmailSendingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Sending verification email was unseccessful!"})),
            )
                .into_response(),
            LoginErrorResponse::UserCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Creating user was unsuccessful!"})),
            )
                .into_response(),
            LoginErrorResponse::Hashing => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Hashing error"})),
            )
                .into_response(),
            LoginErrorResponse::SignUpError(signup_error) => {
                (StatusCode::BAD_REQUEST, Json(signup_error.deref())).into_response()
            }
            LoginErrorResponse::WrongOtp => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Wrong OTP"})),
            ).into_response(),
            LoginErrorResponse::AuthError(ae) => ae.into_response(),
        }
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
