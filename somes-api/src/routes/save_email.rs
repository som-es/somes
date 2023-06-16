use axum::Json;
use somes_common_lib::SaveEmailInfo;

pub async fn save_email(Json(save_email_info): Json<SaveEmailInfo>) -> Result<Json<()>, String> {
    println!("save_email_info: {save_email_info:?}");

    Ok(Json(()))
}
