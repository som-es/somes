use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema, Debug)]
pub enum LegisInitErrorResponse {
    LegisInit,
    LatestLegisInit,
    LatestVoteResults,
    InvalidPage
}

impl IntoResponse for LegisInitErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            LegisInitErrorResponse::LegisInit => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return legislative initiatives",
            ),
            LegisInitErrorResponse::LatestLegisInit => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return latest legislative initiatives",
            ),
            LegisInitErrorResponse::LatestVoteResults => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return latest vote results",
            ),
            LegisInitErrorResponse::InvalidPage => (
                StatusCode::BAD_REQUEST,
                "invalid page"
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
