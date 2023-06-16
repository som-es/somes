use axum::Json;
mod error;

use crate::{
    dataservice::{dataservice_con, get_speakers_by_hours},
    model::SpeakerByHours,
};

use self::error::SpeakerByHoursErrorResponse;

pub async fn speaker_by_hours() -> Result<Json<Vec<SpeakerByHours>>, SpeakerByHoursErrorResponse> {
    get_speakers_by_hours(&mut dataservice_con())
        .map_err(|_| SpeakerByHoursErrorResponse::DbSelectFailure)
        .map(Json)
}
