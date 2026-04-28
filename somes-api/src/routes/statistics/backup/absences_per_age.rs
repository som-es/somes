use axum::Json;

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
pub struct AgeAbcenesFilter {
    legis_period: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeAbcenes {
    age_group: String,
    age_group_members_with_abcenses: i64,
    total_absences: i64,
    normalized_absences: f64,
}

// #[debug_handler]
pub async fn absences_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeAbcenesFilter>>,
) -> Result<Json<Vec<AgeAbcenes>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter
        .legis_period
        .with_sql_column("dga.legislative_period");
    let filter_arg1 = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg2 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_absences"
    } else {
        "total_absences"
    };

    let filter = build_filter(&filters);

    let query = format!(
        " 
SELECT 
    CASE 
        WHEN dga.age_at_start < 30 THEN 'Under 30'
        WHEN dga.age_at_start BETWEEN 30 AND 39 THEN '30-39'
        WHEN dga.age_at_start BETWEEN 40 AND 49 THEN '40-49'
        WHEN dga.age_at_start BETWEEN 50 AND 59 THEN '50-59'
        WHEN dga.age_at_start BETWEEN 60 AND 69 THEN '60-69'
        ELSE '70+'
    END AS age_group,
    COUNT(DISTINCT ds.id) AS age_group_members_with_abcenses,
    COUNT(DISTINCT ab.id) AS total_absences,
    COUNT(DISTINCT ab.id)::FLOAT / COUNT(DISTINCT ds.id)::FLOAT AS normalized_absences
FROM 
    absences ab
JOIN 
    delegates ds ON ab.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    plenar_infos pf ON pf.id = ab.plenary_session_id
JOIN 
    delegate_ages dga ON dga.delegate_id = ds.id
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
GROUP BY 
    age_group
ORDER BY 
    {normalized} {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, AgeAbcenes>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
