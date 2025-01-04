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
pub struct AgeTotalSpeechesFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeTotalSpeeches {
    age_group: String,
    total_speeches: i64,
    age_group_members_with_speeches: i64,
    normalized_speeches: f64,
}

// #[debug_handler]
pub async fn total_speeches_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeTotalSpeechesFilter>>,
) -> Result<Json<Vec<AgeTotalSpeeches>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
            CASE 
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) < 30 THEN 'Under 30'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 30 AND 39 THEN '30-39'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 40 AND 49 THEN '40-49'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 50 AND 59 THEN '50-59'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 60 AND 69 THEN '60-69'
                ELSE '70+'
            END AS age_group,
            COUNT(ps.id) AS total_speeches,
            COUNT(DISTINCT ds.id) AS age_group_members_with_speeches,
            COUNT(ps.id)::FLOAT / COUNT(DISTINCT ds.id)::FLOAT AS normalized_speeches
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
    age_group
ORDER BY 
    normalized_speeches {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, AgeTotalSpeeches>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
