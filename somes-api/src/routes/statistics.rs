use axum::Json;
mod error;

use crate::{
    dataservice::{dataservice_con, get_speakers_by_hours, get_delegates_by_call_to_orders},
    model::{SpeakerByHours, DelegateByCallToOrders},
};

use self::error::StatisticsResponse;

pub async fn speakers_by_hours() -> Result<Json<Vec<SpeakerByHours>>, StatisticsResponse> {
    get_speakers_by_hours(&mut dataservice_con())
        .map_err(|_| StatisticsResponse::DbSelectFailure)
        .map(Json)
}

pub async fn delegate_by_call_to_orders() -> Result<Json<Vec<DelegateByCallToOrders>>, StatisticsResponse> {
    get_delegates_by_call_to_orders(&mut dataservice_con())
        .map_err(|_| StatisticsResponse::DbSelectFailure)
        .map(Json)
}
