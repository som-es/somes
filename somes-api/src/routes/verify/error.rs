use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

use crate::jwt::AuthError;

#[derive(ToSchema, Debug)]
pub enum VerifyErrorResponse {
    InvalidVerificationID,
    VerificationError,
    UserCreationError,
    Auth(AuthError),
}

impl IntoResponse for VerifyErrorResponse {
    fn into_response(self) -> axum::response::Response {
        // rather returning an entire struct? like in signup?
        let (status_code, err_msg) = match self {
            VerifyErrorResponse::InvalidVerificationID => {
                (StatusCode::BAD_REQUEST, "Invalid verification id provided.")
            }
            VerifyErrorResponse::VerificationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "verification failed")
            }
            VerifyErrorResponse::UserCreationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "creating user failed")
            }
            VerifyErrorResponse::Auth(err) => return err.into_response(),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
