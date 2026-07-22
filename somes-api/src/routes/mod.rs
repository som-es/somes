mod dates;
mod decrees;
mod delegates;
mod departments;
mod events;
mod parties;
mod plenar;
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

use chrono::NaiveDate;
use combx::with_data::gps::LegislativePeriod;
use combx::Parliament;
pub use dates::*;
pub use decrees::*;
pub use delegates::*;
pub use departments::*;
pub use events::*;
pub use parties::*;
pub use plenar::*;
pub use proposals::*;
pub use quiz::*;
pub use save_email::*;
pub use statistics::create_statistics_router;
pub use statistics::fetch_latest_session_activity_overview;
pub use statistics::session_activity::CACHE_KEY as SESSION_ACTIVITY_CACHE_KEY;
pub use topics::*;
pub use user::*;
pub use verify::*;
pub use vote_results::*;
pub use walo::*;

use crate::ParliamentCtx;
use crate::{GenericError, PgPoolConnection};
use axum::Json;

pub async fn all_gps_route(
    ParliamentCtx(parliament): ParliamentCtx,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<LegislativePeriod>>, GenericError> {
    // if parliament == Parliament::Eu {
    //     return Ok(Json(vec![
    //         LegislativePeriod {
    //             gp: "10".into(),
    //             start_date: NaiveDate::from_ymd_opt(2026, 7, 16).unwrap(),
    //         },
    //         LegislativePeriod {
    //             gp: "9".into(),
    //             start_date: NaiveDate::from_ymd_opt(2019, 7, 2).unwrap(),
    //         },
    //     ]));
    // }
    Ok(Json(
        combx::with_data::gps::gps(&pg)
            .await
            .map_err(|e| GenericError::SqlFailure(Some(e)))?,
    ))
}
