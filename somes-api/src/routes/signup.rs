use axum::Json;
use somes_common_lib::{JWTInfo, SignUpInfo};

use self::error::SignUpError;

mod error;

pub async fn signup(Json(payload): Json<SignUpInfo>) -> Result<Json<JWTInfo>, SignUpError> {
    todo!()
}
