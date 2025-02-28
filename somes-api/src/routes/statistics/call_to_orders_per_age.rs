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
pub struct AgeCallToOrdersFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeCallToOrders {
    age_group: String,
    age_group_members_who_where_called: i64,
    total_order_calls: i64,
    normalized_calls_to_order: f64,
}

// #[debug_handler]
pub async fn call_to_orders_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeCallToOrdersFilter>>,
) -> Result<Json<Vec<AgeCallToOrders>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

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
            CASE 
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) * (-1) < 30 THEN 'Under 30'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) * (-1) BETWEEN 30 AND 39 THEN '30-39'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) * (-1) BETWEEN 40 AND 49 THEN '40-49'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) * (-1) BETWEEN 50 AND 59 THEN '50-59'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate, lpd.start_date)) * (-1) BETWEEN 60 AND 69 THEN '60-69'
                ELSE '70+'
            END AS age_group,
            COUNT(DISTINCT ds.id) AS age_group_members_who_where_called,
            COUNT(cto.id) AS total_order_calls,
            COUNT(cto.id)::FLOAT / COUNT(DISTINCT ds.id)::FLOAT AS normalized_calls_to_order
        FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            legislative_period_dates lpd ON lpd.legislative_period = pf.legislative_period
        WHERE 
            {filter}    
            AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
            AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
        GROUP BY 
            age_group
        ORDER BY 
            normalized_calls_to_order {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, AgeCallToOrders>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
