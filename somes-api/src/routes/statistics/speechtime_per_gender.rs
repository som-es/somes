use axum::{debug_handler, Json};

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::{
        error::StatisticsResponse,
        filtering::{bind_values, build_filter, IntoFilterArgument},
    },
    PgPoolConnection,
};

use super::filtering::Manual;

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenderSpeechTimeFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenderSpeechTime {
    delegate_gender: String,
    gender_members: i64,
    total_speech_time: i64,
    normalized_speech_time: f64,
}

// #[debug_handler]
pub async fn speechtime_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<GenderSpeechTimeFilter>>,
) -> Result<Json<Vec<GenderSpeechTime>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        WITH gender_counts AS (
    SELECT 
        ds.gender, 
        COUNT(DISTINCT ds.id) AS total_gender_count
    FROM delegates ds
    JOIN mandates m ON m.delegate_id = ds.id
    JOIN plenar_infos pf ON 1=1  
    WHERE {filter}
        AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
        AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
    GROUP BY ds.gender
)
        SELECT 
            ds.gender AS delegate_gender,
            gc.total_gender_count AS gender_members,
            SUM(ps.duration_in_seconds) / 60 AS total_speech_time,
            (SUM(ps.duration_in_seconds)::FLOAT / gc.total_gender_count::FLOAT) / 60 AS normalized_speech_time
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
    gender_counts gc ON ds.gender = gc.gender
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
    ds.gender, gc.total_gender_count
ORDER BY 
    normalized_speech_time {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, GenderSpeechTime>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
