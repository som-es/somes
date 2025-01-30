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
pub struct LegisAgeFilter {
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegisAge {
    legislative_period: String,
    average_age: f64,
}

// #[debug_handler]
pub async fn age_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegisAgeFilter>>,
) -> Result<Json<Vec<LegisAge>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.party.with_sql_column("m.party");
    let filter_arg1 = filter.gender.with_sql_column("ds.gender");
    let filter_arg2 = Manual("m.is_nr").with_sql_column("");
    let filter_arg3 = Manual("birthdate is not null").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

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
SELECT DISTINCT 
     pf.legislative_period AS legislative_period,
    AVG(
    (EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) + 
    (EXTRACT(MONTH FROM AGE(ds.birthdate, lpd.start_date)) / 12.0) + 
    (EXTRACT(DAY FROM AGE(ds.birthdate, lpd.start_date)) / 365.25))::FLOAT  
    ) * (-1) as average_age  

        FROM 
            delegates ds
        JOIN 
            plenar_speeches ps ON ps.delegate_id = ds.id
        JOIN 
            debates db ON db.id = ps.debate_id
        JOIN 
            plenar_infos pf ON pf.id = db.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            legislative_period_dates lpd ON lpd.legislative_period = pf.legislative_period
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
GROUP BY 
     pf.legislative_period
ORDER BY 
    average_age {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegisAge>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
