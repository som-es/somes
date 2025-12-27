use axum::{extract::Query, Json};
use chrono::{Months, NaiveDateTime, NaiveTime};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use somes_common_lib::Date;

use crate::{today_and_time, GenericError, PgPoolConnection};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlenarDate {
    pub date_and_time: NaiveDateTime,
}

pub async fn next_plenar_date_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<PlenarDate>, GenericError> {
    let now = today_and_time();
    sqlx::query!(
        "select date, time from dates where start >= $1 and appointment_type = 'Plenarsitzung' and committee = 'Nationalrat' order by date asc limit 1",
        now
    )
    .fetch_one(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))
    .map(|date_time| {
       PlenarDate { date_and_time: NaiveDateTime::new(date_time.date, date_time.time.unwrap_or(NaiveTime::default())) }
    })
    .map(Json)
}

pub async fn plenar_dates_route(
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<PlenarDate>>, GenericError> {
    let date_before = date
        .at
        .checked_sub_months(Months::new(2))
        .ok_or(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "Invalid date",
        )))?;
    let date_after = date
        .at
        .checked_add_months(Months::new(2))
        .ok_or(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "Invalid date",
        )))?;

    sqlx::query!(
        "select date, time from dates where date >= $1 and date <= $2 and appointment_type = 'Plenarsitzung' and committee = 'Nationalrat'",
        date_before,
        date_after
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))
    .map(|date_times| {
           date_times.into_iter().map(|date_time| PlenarDate { date_and_time: NaiveDateTime::new(date_time.date, date_time.time.unwrap_or(NaiveTime::default())) }).collect()
        })
    .map(Json)
}
