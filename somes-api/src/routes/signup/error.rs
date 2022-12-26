use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum SignUpError {
    MissingUsername,
    MissingPassword,
    MissingEmail,
    UsernameTaken,
    EMailTaken,
    InvalidEmail,
    InsufficientPassword,
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SignUpError::MissingUsername => (StatusCode::BAD_REQUEST, "Missing username."),
            SignUpError::MissingPassword => (StatusCode::BAD_REQUEST, "Missing password."),
            SignUpError::MissingEmail => (StatusCode::BAD_REQUEST, "Missing email."),
            SignUpError::UsernameTaken => (StatusCode::BAD_REQUEST, "Username is taken."),
            SignUpError::EMailTaken => (StatusCode::BAD_REQUEST, "E-mail is taken."),
            SignUpError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid e-mail provided."),
            SignUpError::InsufficientPassword => (
                StatusCode::BAD_REQUEST,
                "Password does not comply with standards.",
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
