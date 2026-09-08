mod dates;
mod decrees;
mod delegates;
mod departments;
mod events;
mod orientation_questions;
mod parties;
mod plenar;
mod proposals;
mod questions;
mod quiz;
mod save_email;
mod speeches;
mod statistics;
mod topics;
mod user;
mod verify;
mod volksbg;
mod vote_results;
mod walo;

use combx::with_data::gps::LegislativePeriod;
pub use dates::*;
pub use decrees::*;
pub use delegates::*;
pub use departments::*;
pub use events::*;
pub use orientation_questions::*;
pub use parties::*;
pub use plenar::*;
pub use proposals::*;
pub use quiz::*;
pub use save_email::*;
pub use speeches::*;
pub use statistics::create_statistics_router;
pub use statistics::fetch_latest_session_activity_overview;
pub use statistics::session_activity::CACHE_KEY as SESSION_ACTIVITY_CACHE_KEY;
pub use topics::*;
pub use user::*;
pub use verify::*;
pub use volksbg::*;
pub use vote_results::*;
pub use walo::*;

use crate::{GenericError, PgPoolConnection};
use axum::Json;

pub async fn all_gps_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<LegislativePeriod>>, GenericError> {
    Ok(Json(
        combx::with_data::gps::gps(&pg)
            .await
            .map_err(|e| GenericError::SqlFailure(Some(e)))?,
    ))
}
