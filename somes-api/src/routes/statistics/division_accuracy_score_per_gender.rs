use axum::{debug_handler, Json};

use diesel::sql_types::Bool;
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
pub struct GenderDivisonAccuracyFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenderDivisionAccuracy {
    gender: String,
    average_division_accuracy_score: f64,
}

// #[debug_handler]
pub async fn division_accuracy_score_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<GenderDivisonAccuracyFilter>>,
) -> Result<Json<Vec<GenderDivisionAccuracy>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
        SELECT 
            ds.gender AS gender,
            AVG(dis.score) AS average_division_accuracy_score
         FROM 
            division_interest_score dis
        JOIN 
            delegates ds ON dis.delegate_id = ds.id
        JOIN 
            plenar_speeches ps ON ps.delegate_id = ds.id
        JOIN 
            debates db ON db.id = ps.debate_id
        JOIN 
            plenar_infos pf ON pf.id = db.plenar_id
         JOIN 
            mandates m ON m.delegate_id = ds.id
        WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY
        ds.gender
ORDER BY 
    average_division_accuracy_score {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, GenderDivisionAccuracy>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
