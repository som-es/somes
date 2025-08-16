use axum::Json;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    get_json_cache,
    routes::statistics::{
        error::StatisticsResponse,
        filtering::{bind_values, build_filter, IntoFilterArgument},
    },
    set_json_cache_secs, PgPoolConnection, RedisConnection,
};

use super::filtering::Manual;

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DelegateSpeechTimeFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateSpeechTime {
    delegate_name: String,
    delegate_party: String,
    total_speech_time: i64,
    total_sessions_attended: i64,
    normalized_speech_time: f64,
}

// #[debug_handler]
pub async fn speechtime_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_client): RedisConnection,
    Json(filter): Json<Option<DelegateSpeechTimeFilter>>,
) -> Result<Json<Vec<DelegateSpeechTime>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let key = format!("speechtime_per_delegate/{:?}", filter);
    if let Some(entry) = get_json_cache(&mut redis_client, &key).await {
        return Ok(Json(entry));
    }

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("m.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_speech_time"
    } else {
        "total_speech_time"
    };

    let filter = build_filter(&filters);

    let query = format!(
        "
         WITH legislative_period_dates AS (
    SELECT 
        legislative_period, 
        MIN(add_date) AS start_date, 
        MAX(add_date) AS end_date
    FROM 
        plenar_infos
    GROUP BY 
        legislative_period
),
session_counts AS (
    SELECT 
        pf.legislative_period,
        ps.delegate_id,
        COUNT(DISTINCT pf.id) AS total_sessions_attended
    FROM 
        plenar_infos pf
    JOIN 
        debates db ON db.plenar_id = pf.id
    JOIN 
        plenar_speeches ps ON ps.debate_id = db.id
    GROUP BY 
        pf.legislative_period, ps.delegate_id
)
SELECT 
    ds.name AS delegate_name,
    COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
    SUM(ps.duration_in_seconds) / 60 AS total_speech_time,
    sc.total_sessions_attended,
    (SUM(ps.duration_in_seconds) / 60) / NULLIF(sc.total_sessions_attended, 0)::FLOAT AS normalized_speech_time
FROM 
    plenar_speeches ps
JOIN 
    delegates ds ON ps.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    debates db ON db.id = ps.debate_id
JOIN 
    plenar_infos pf ON pf.id = db.plenar_id
JOIN 
    legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
JOIN 
    session_counts sc ON sc.legislative_period = lp.legislative_period 
    AND sc.delegate_id = ds.id  
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
    ds.name, m.party, sc.total_sessions_attended
ORDER BY 
    {normalized} {desc};

    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateSpeechTime>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    let data = filtered_query
        .fetch_all(&pg)
        .await
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    set_json_cache_secs(&mut redis_client, &key, &data, 60 * 60 * 6).await;
    Ok(Json(data))
}
