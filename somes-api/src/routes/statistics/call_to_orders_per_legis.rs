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
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegisCallToOrders {
    legislative_period: String,
    total_order_calls: i64,
    total_sessions: i64,
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
    let filter_arg2 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_calls_to_order"
    } else {
        "total_order_calls"
    };

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
), session_counts AS (
    SELECT 
        pf.legislative_period, 
        COUNT(DISTINCT pf.id) AS total_sessions
    FROM 
        plenar_infos pf
    JOIN 
        absences ab ON ab.plenary_session_id = pf.id
    GROUP BY 
        pf.legislative_period
)
        SELECT 
            pf.legislative_period AS legislative_period,
            COUNT(cto.id) AS total_order_calls,
            sc.total_sessions AS total_sessions,
            COUNT(cto.id)::FLOAT / sc.total_sessions::FLOAT AS normalized_calls_to_order
        FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            legislative_period_dates lp ON pf.legislative_period = lp.legislative_period
        JOIN 
            session_counts sc ON sc.legislative_period = lp.legislative_period 
        WHERE 
            {filter}    
            AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
            AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
        GROUP BY 
            pf.legislative_period, sc.total_sessions
        ORDER BY 
            {normalized} {desc};
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
