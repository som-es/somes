use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum LegisInitErrorResponse {
    LegisInitError,
    LatestLegisInitError,
    LatestVoteResultsError,
}

impl IntoResponse for LegisInitErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            LegisInitErrorResponse::LegisInitError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return legislative initiatives",
            ),
            LegisInitErrorResponse::LatestLegisInitError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return latest legislative initiatives",
            ),
            LegisInitErrorResponse::LatestVoteResultsError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return latest vote results",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
