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
pub struct PartySpeechTimeFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartySpeechTime {
    delegate_party: String,
    party_members_with_speeche_time: i64,
    total_speech_time: i64,
    normalized_speech_time: f64,
}

// #[debug_handler]
pub async fn speechtime_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartySpeechTimeFilter>>,
) -> Result<Json<Vec<PartySpeechTime>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
            m.party AS delegate_party,
            COUNT(DISTINCT ds.id) AS party_members_with_speeche_time,
            SUM(ps.duration_in_seconds) AS total_speech_time,
            SUM(ps.duration_in_seconds)::FLOAT / COUNT(DISTINCT ds.id)::FLOAT AS normalized_speech_time
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
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
    m.party
ORDER BY 
    normalized_speech_time {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartySpeechTime>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
