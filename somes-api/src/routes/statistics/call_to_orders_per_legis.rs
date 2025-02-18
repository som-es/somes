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
pub struct LegisCallToOrdersFilter {
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegisCallToOrders {
    legislative_period: String,
    total_order_calls: i64,
    period_duration_days: f64,
    normalized_calls_to_order: f64,
}

// #[debug_handler]
pub async fn call_to_orders_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegisCallToOrdersFilter>>,
) -> Result<Json<Vec<LegisCallToOrders>>, StatisticsResponse> {
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
            COUNT(cto.id) AS total_order_calls,
            EXTRACT(DAY FROM (ld.end_date - ld.start_date))::FLOAT AS period_duration_days,
            COUNT(cto.id)::FLOAT / NULLIF(EXTRACT(DAY FROM (ld.end_date - ld.start_date)), 0)::FLOAT AS normalized_calls_to_order
        FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            legislative_period_dates ld ON pf.legislative_period = ld.legislative_period
        WHERE 
            {filter}    
            AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
            AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
        GROUP BY 
            pf.legislative_period, ld.start_date, ld.end_date
        ORDER BY 
            normalized_calls_to_order {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegisCallToOrders>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
