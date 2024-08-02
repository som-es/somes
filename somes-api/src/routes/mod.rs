mod delegates;
mod legislative_initiatives;
mod login;
mod parties;
mod questions;
mod reset_password;
mod save_email;
mod signup;
mod statistics;
mod user;
mod verify;

use axum::Json;
pub use delegates::*;
pub use legislative_initiatives::*;
pub use login::*;
pub use parties::*;
pub use questions::*;
pub use reset_password::*;
pub use save_email::*;
use serde_json::json;
pub use signup::*;
pub use statistics::*;
pub use user::*;
pub use verify::*;

use crate::PgPoolConnection;

pub async fn all_gps(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<dataservice::with_data::gps::LegislativePeriod>>, Json<serde_json::Value>> {
    Ok(Json(dataservice::with_data::gps::gps(&pg).await.map_err(
        |_| Json(json!({"error": "could not return all legislative periods"})),
    )?))
}
