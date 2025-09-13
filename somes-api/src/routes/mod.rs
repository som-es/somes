mod dates;
mod decrees;
mod delegates;
mod favo;
mod legislative_initiatives;
mod login;
mod mail_send_info;
mod parties;
mod proposals;
mod questions;
mod quiz;
mod reset_password;
mod save_email;
mod signup;
mod statistics;
mod topics;
mod user;
mod verify;
mod walo;

pub use proposals::*;

pub use favo::*;

use axum::Json;
pub use dates::*;
pub use decrees::*;
pub use delegates::*;
pub use legislative_initiatives::*;
pub use login::*;
pub use mail_send_info::*;
pub use parties::*;
pub use quiz::*;
pub use reset_password::*;
pub use save_email::*;
use serde_json::json;
pub use signup::*;
pub use statistics::*;
pub use topics::*;
pub use user::*;
pub use verify::*;
pub use walo::*;

use crate::PgPoolConnection;

pub async fn all_gps(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<dataservice::combx::with_data::gps::LegislativePeriod>>, Json<serde_json::Value>>
{
    Ok(Json(
        dataservice::combx::with_data::gps::gps(&pg)
            .await
            .map_err(|_| Json(json!({"error": "could not return all legislative periods"})))?,
    ))
}
