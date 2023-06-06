use std::ops::{Deref, DerefMut};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use somes_common_lib::errors::SignUpError;

#[derive(Debug)]
pub enum SignUpErrorResponse {
    VerificationEmailSendingError,
    UserCreationError,
    SignUpError(SignUpErrorWrapper),
}

impl IntoResponse for SignUpErrorResponse {
    fn into_response(self) -> Response {
        match self {
            SignUpErrorResponse::VerificationEmailSendingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Sending verification email was unseccessful!"})),
            )
                .into_response(),
            SignUpErrorResponse::UserCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Creating user was unsuccessful!"})),
            )
                .into_response(),
            SignUpErrorResponse::SignUpError(signup_error) => {
                (StatusCode::BAD_REQUEST, Json(signup_error.deref())).into_response()
            }
        }
    }
}

#[derive(Debug, Default)]
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
