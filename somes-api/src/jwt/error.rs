use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

use crate::ErrorInfo;

#[derive(ToSchema, Debug, Clone, Copy)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    MissingToken,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::MissingToken => (StatusCode::BAD_REQUEST, "Missing/Invalid token"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        
        let body = Json(ErrorInfo {
            error: error_message.to_string(),
            error_type: "AuthError",
            field: format!("{:?}", self),
            meta: None,
        });
        (status, body).into_response()
    }
}
