use axum::Json;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::{routes::statistics::routes::error::StatisticsResponse, PgPoolConnection};

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionRow {
    plenary_session_id: i32,
    date: Option<DateTime<Utc>>,
    legislative_period: Option<String>,
    inr: Option<i32>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionActivityMetrics {
    vote_count: i64,
    call_to_order_count: i64,
    speaker_count: i64,
    speech_count: i64,
    total_speech_time: i64,
    absence_count: i64,
    average_complexity: f64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionActivityPercentiles {
    vote_count_p95: f64,
    speaker_count_p95: f64,
    absence_count_p95: f64,
    delegate_speech_time_p95: f64,
    complexity_p95: f64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionSpeaker {
    delegate_name: String,
    delegate_party: String,
    total_speeches: i64,
    total_speech_time: i64,
    longest_speech_time: i32,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionCallToOrder {
    delegate_name: String,
    delegate_party: String,
    total_order_calls: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SessionActivityOverview {
    plenary_session_id: i32,
    date: Option<DateTime<Utc>>,
    legislative_period: Option<String>,
    inr: Option<i32>,
    vote_count: i64,
    call_to_order_count: i64,
    speaker_count: i64,
    speech_count: i64,
    total_speech_time: i64,
    absence_count: i64,
    average_complexity: f64,
    percentiles: SessionActivityPercentiles,
    top_speakers: Vec<SessionSpeaker>,
    call_to_orders: Vec<SessionCallToOrder>,
}

fn initiative_complexity_sql(alias: &str) -> String {
    format!(
        "
        CASE
            WHEN {alias}.ityp = 'J' THEN 1.0
            WHEN {alias}.ityp = 'AA' THEN 1.2
            WHEN {alias}.ityp = 'A' THEN 1.2
            WHEN {alias}.ityp = 'UEA' THEN 1.15
            WHEN {alias}.ityp = 'I' THEN 1.3
            ELSE 1.0
        END
        "
    )
}

pub async fn latest_session_activity_overview(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Option<SessionActivityOverview>>, StatisticsResponse> {
    let latest_session = sqlx::query_as::<_, SessionRow>(
        "
        SELECT
            pi.id AS plenary_session_id,
            pi.raw_data_created_at AS date,
            pi.legislative_period,
            pi.inr
        FROM plenar_infos pi
        JOIN legislative_initiatives li ON li.plenary_session_id = pi.id
        WHERE li.is_voteable_on
            AND li.accepted IS NOT NULL
        ORDER BY li.nr_plenary_activity_date DESC, pi.inr DESC
        LIMIT 1
        ",
    )
    .fetch_optional(&pg)
    .await
    .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    let Some(session) = latest_session else {
        return Ok(Json(None));
    };

    let metrics = sqlx::query_as::<_, SessionActivityMetrics>(&format!(
        "
        SELECT
            (
                SELECT COUNT(DISTINCT li.id)
                FROM legislative_initiatives li
                WHERE li.plenary_session_id = $1
                    AND li.is_voteable_on
                    AND li.accepted IS NOT NULL
            ) AS vote_count,
            (
                SELECT COUNT(DISTINCT cto.id)
                FROM call_to_order cto
                WHERE cto.plenar_id = $1
            ) AS call_to_order_count,
            (
                SELECT COUNT(DISTINCT ps.delegate_id)
                FROM plenar_speeches ps
                JOIN debates db ON db.id = ps.debate_id
                WHERE db.plenar_id = $1
            ) AS speaker_count,
            (
                SELECT COUNT(DISTINCT ps.id)
                FROM plenar_speeches ps
                JOIN debates db ON db.id = ps.debate_id
                WHERE db.plenar_id = $1
            ) AS speech_count,
            (
                SELECT COALESCE(SUM(ps.duration_in_seconds), 0)
                FROM plenar_speeches ps
                JOIN debates db ON db.id = ps.debate_id
                WHERE db.plenar_id = $1
            ) AS total_speech_time,
            (
                SELECT COUNT(DISTINCT ab.id)
                FROM absences ab
                WHERE ab.plenary_session_id = $1
            ) AS absence_count,
            (
                SELECT COALESCE(AVG({}), 0)::FLOAT8
                FROM legislative_initiatives li
                WHERE li.plenary_session_id = $1
                    AND li.is_voteable_on
                    AND li.accepted IS NOT NULL
            ) AS average_complexity
        ",
        initiative_complexity_sql("li")
    ))
    .bind(session.plenary_session_id)
    .fetch_one(&pg)
    .await
    .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    let percentiles = sqlx::query_as::<_, SessionActivityPercentiles>(&format!(
        "
        SELECT
            COALESCE((
                SELECT percentile_cont(0.95) WITHIN GROUP (ORDER BY vote_count)::FLOAT8
                FROM (
                    SELECT COUNT(DISTINCT li.id) AS vote_count
                    FROM legislative_initiatives li
                    WHERE li.is_voteable_on
                        AND li.accepted IS NOT NULL
                        AND li.plenary_session_id IS NOT NULL
                    GROUP BY li.plenary_session_id
                ) vote_counts
            ), 0) AS vote_count_p95,
            COALESCE((
                SELECT percentile_cont(0.95) WITHIN GROUP (ORDER BY speaker_count)::FLOAT8
                FROM (
                    SELECT COUNT(DISTINCT ps.delegate_id) AS speaker_count
                    FROM plenar_speeches ps
                    JOIN debates db ON db.id = ps.debate_id
                    GROUP BY db.plenar_id
                ) speaker_counts
            ), 0) AS speaker_count_p95,
            COALESCE((
                SELECT percentile_cont(0.95) WITHIN GROUP (ORDER BY absence_count)::FLOAT8
                FROM (
                    SELECT COUNT(DISTINCT ab.id) AS absence_count
                    FROM absences ab
                    GROUP BY ab.plenary_session_id
                ) absence_counts
            ), 0) AS absence_count_p95,
            COALESCE((
                SELECT percentile_cont(0.95) WITHIN GROUP (ORDER BY delegate_speech_time)::FLOAT8
                FROM (
                    SELECT COALESCE(SUM(ps.duration_in_seconds), 0) AS delegate_speech_time
                    FROM plenar_speeches ps
                    JOIN debates db ON db.id = ps.debate_id
                    WHERE ps.duration_in_seconds IS NOT NULL
                    GROUP BY db.plenar_id, ps.delegate_id
                ) delegate_speech_times
            ), 0) AS delegate_speech_time_p95,
            COALESCE((
                SELECT percentile_cont(0.95) WITHIN GROUP (ORDER BY average_complexity)::FLOAT8
                FROM (
                    SELECT AVG({})::FLOAT8 AS average_complexity
                    FROM legislative_initiatives li
                    WHERE li.is_voteable_on
                        AND li.accepted IS NOT NULL
                        AND li.plenary_session_id IS NOT NULL
                    GROUP BY li.plenary_session_id
                ) complexity_scores
            ), 0) AS complexity_p95
        ",
        initiative_complexity_sql("li")
    ))
    .fetch_one(&pg)
    .await
    .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    let top_speakers = sqlx::query_as::<_, SessionSpeaker>(
        "
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            COUNT(ps.id) AS total_speeches,
            COALESCE(SUM(ps.duration_in_seconds), 0) AS total_speech_time,
            COALESCE(MAX(ps.duration_in_seconds), 0) AS longest_speech_time
        FROM plenar_speeches ps
        JOIN delegates d ON d.id = ps.delegate_id
        JOIN debates db ON db.id = ps.debate_id
        JOIN plenar_infos pf ON pf.id = db.plenar_id
        LEFT JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        WHERE db.plenar_id = $1
            AND ps.duration_in_seconds IS NOT NULL
        GROUP BY d.id, d.name, m.party
        ORDER BY total_speech_time DESC
        LIMIT 3
        ",
    )
    .bind(session.plenary_session_id)
    .fetch_all(&pg)
    .await
    .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    let call_to_orders = sqlx::query_as::<_, SessionCallToOrder>(
        "
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            COUNT(cto.id) AS total_order_calls
        FROM call_to_order cto
        JOIN delegates d ON d.id = cto.receiver_id
        JOIN plenar_infos pf ON pf.id = cto.plenar_id
        LEFT JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        WHERE cto.plenar_id = $1
        GROUP BY d.id, d.name, m.party
        ORDER BY total_order_calls DESC, d.name ASC
        LIMIT 3
        ",
    )
    .bind(session.plenary_session_id)
    .fetch_all(&pg)
    .await
    .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    Ok(Json(Some(SessionActivityOverview {
        plenary_session_id: session.plenary_session_id,
        date: session.date,
        legislative_period: session.legislative_period,
        inr: session.inr,
        vote_count: metrics.vote_count,
        call_to_order_count: metrics.call_to_order_count,
        speaker_count: metrics.speaker_count,
        speech_count: metrics.speech_count,
        total_speech_time: metrics.total_speech_time,
        absence_count: metrics.absence_count,
        average_complexity: metrics.average_complexity,
        percentiles,
        top_speakers,
        call_to_orders,
    })))
}
