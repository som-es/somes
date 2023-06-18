use axum::{Json, extract::State};
mod error;

use crate::{
    dataservice::{dataservice_con, get_speakers_by_hours, get_delegates_by_call_to_orders},
    model::{SpeakerByHours, DelegateByCallToOrders}, server::AppState, PostgresConnection,
};

use self::error::StatisticsResponse;

pub async fn speakers_by_hours() -> Result<Json<Vec<SpeakerByHours>>, StatisticsResponse> {
    get_speakers_by_hours(&mut dataservice_con())
        .map_err(|_| StatisticsResponse::DbSelectFailure)
        .map(Json)
}

pub async fn delegate_by_call_to_orders() -> Result<Json<Vec<DelegateByCallToOrders>>, StatisticsResponse> {
    todo!()
}

/* 
pub async fn delegate_by_call_to_orders(PostgresConnection(postgres_con): PostgresConnection) -> Result<Json<Vec<DelegateByCallToOrders>>, StatisticsResponse> {

    Ok(postgres_con.interact(|con| {
        Json(get_delegates_by_call_to_orders(con).unwrap())
    }).await.unwrap())


    /* 
    get_delegates_by_call_to_orders(&mut dataservice_con())
        .map_err(|_| StatisticsResponse::DbSelectFailure)
        .map(Json)*/
}
*/