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
pub struct LeigsTotalSpeechesFilter {
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LeigsTotalSpeeches {
    legislative_period: String,
    total_speeches: i64,
    period_duration_days: f64,
    normalized_speeches: f64,
}

// #[debug_handler]
pub async fn total_speeches_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LeigsTotalSpeechesFilter>>,
) -> Result<Json<Vec<LeigsTotalSpeeches>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.party.with_sql_column("m.party");
    let filter_arg1 = filter.gender.with_sql_column("ds.gender");
    let filter_arg2 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

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
)
        SELECT 
            pf.legislative_period AS legislative_period,
            COUNT(ps.id) AS total_speeches,
             EXTRACT(DAY FROM (ld.end_date - ld.start_date))::FLOAT AS period_duration_days,
            (SUM(ps.duration_in_seconds) / 60) / NULLIF(EXTRACT(DAY FROM (ld.end_date - ld.start_date)), 0)::FLOAT AS normalized_speeches
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
    legislative_period_dates ld ON pf.legislative_period = ld.legislative_period
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
    pf.legislative_period, ld.start_date, ld.end_date
ORDER BY 
    total_speeches {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LeigsTotalSpeeches>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
