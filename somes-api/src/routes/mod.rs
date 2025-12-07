mod dates;
mod decrees;
mod delegates;
mod parties;
mod proposals;
mod questions;
mod quiz;
mod save_email;
mod statistics;
mod topics;
mod user;
mod verify;
mod vote_results;
mod walo;

pub use proposals::*;

use axum::Json;
pub use dates::*;
pub use decrees::*;
pub use delegates::*;
pub use parties::*;
pub use quiz::*;
pub use save_email::*;
use serde_json::json;
pub use statistics::*;
pub use topics::*;
pub use user::*;
pub use verify::*;
pub use vote_results::*;
pub use walo::*;

use crate::PgPoolConnection;

pub async fn all_gps_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<dataservice::combx::with_data::gps::LegislativePeriod>>, Json<serde_json::Value>>
{
    Ok(Json(
        dataservice::combx::with_data::gps::gps(&pg)
            .await
            .map_err(|_| Json(json!({"error": "could not return all legislative periods"})))?,
    ))
}
