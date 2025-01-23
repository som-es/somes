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
pub struct GenderTotalSpeechesFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenderTotalSpeeches {
    delegate_gender: String,
    total_speeches: i64,
    gender_members_with_speeches: i64,
    normalized_speeches_per_gender: f64,
}

// #[debug_handler]
pub async fn total_speeches_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<GenderTotalSpeechesFilter>>,
) -> Result<Json<Vec<GenderTotalSpeeches>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
            ds.gender AS delegate_gender,
            COUNT(ps.id) AS total_speeches,
            COUNT(DISTINCT ds.id) AS gender_members_with_speeches,
            COUNT(ps.id)::FLOAT / COUNT(DISTINCT ds.id)::FLOAT AS normalized_speeches_per_gender
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
    ds.gender
ORDER BY 
    normalized_speeches_per_gender {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, GenderTotalSpeeches>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
