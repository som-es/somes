use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub enum UserErrorResponse {
    InvalidUser,
    InteractionFailed,
}

impl IntoResponse for UserErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            UserErrorResponse::InvalidUser => (
                StatusCode::BAD_REQUEST,
                "Invalid user id provided. User does not exist.",
            ),
            UserErrorResponse::InteractionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interaction with database failed.",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
