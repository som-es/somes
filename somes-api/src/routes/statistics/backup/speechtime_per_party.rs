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
pub struct PartySpeechTimeFilter {
    legis_period: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartySpeechTime {
    delegate_party: String,
    party_members: i64,
    total_speech_time: i64,
    normalized_speech_time: f64,
}

// #[debug_handler]
pub async fn speechtime_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_client): RedisConnection,
    Json(filter): Json<Option<PartySpeechTimeFilter>>,
) -> Result<Json<Vec<PartySpeechTime>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let key = format!("speechtime_per_party/{:?}", filter);
    if let Some(entry) = get_json_cache(&mut redis_client, &key).await {
        return Ok(Json(entry));
    }

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_speech_time"
    } else {
        "total_speech_time"
    };

    let filter = build_filter(&filters);

    let query = format!(
        " 
         WITH party_member_counts AS (
    SELECT 
        ds.party, 
        COUNT(DISTINCT ds.id) AS total_party_member_count
    FROM delegates ds
    JOIN mandates m ON m.delegate_id = ds.id
    JOIN plenar_infos pf ON 1=1  
    WHERE {filter}
        AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
        AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
    GROUP BY ds.party
)
        SELECT 
            m.party AS delegate_party,
            pmc.total_party_member_count AS party_members,
            SUM(ps.duration_in_seconds) / 60 AS total_speech_time,
            (SUM(ps.duration_in_seconds)::FLOAT / pmc.total_party_member_count::FLOAT) / 60 AS normalized_speech_time
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
    party_member_counts pmc ON ds.party = pmc.party
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
    m.party, pmc.total_party_member_count
ORDER BY 
    {normalized} {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartySpeechTime>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    let data = filtered_query
        .fetch_all(&pg)
        .await
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

    set_json_cache_secs(&mut redis_client, &key, &data, 60 * 60 * 6).await;
    Ok(Json(data))
}
