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
pub struct DelegateCallToOrdersFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateCallToOrders {
    delegate_name: String,
    delegate_party: String,
    total_order_calls: i64,
    total_sessions_attended: i64,
    normalized_calls_to_order: f64,
}

// #[debug_handler]
pub async fn call_to_orders_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateCallToOrdersFilter>>,
) -> Result<Json<Vec<DelegateCallToOrders>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("m.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized { "normalized_calls_to_order" } else { "total_order_calls" };

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
),
session_counts AS (
    SELECT 
        pf.legislative_period,
        ps.delegate_id,
        COUNT(DISTINCT pf.id) AS total_sessions_attended
    FROM 
        plenar_infos pf
    JOIN 
        debates db ON db.plenar_id = pf.id
    JOIN 
        plenar_speeches ps ON ps.debate_id = db.id
    GROUP BY 
        pf.legislative_period, ps.delegate_id
)
       SELECT 
            ds.name AS delegate_name,
            m.party AS delegate_party,
            COUNT(cto.id) AS total_order_calls,
            sc.total_sessions_attended,
            COUNT(DISTINCT cto.id)::FLOAT / NULLIF(sc.total_sessions_attended, 0)::FLOAT AS normalized_calls_to_order
         FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
        JOIN 
            session_counts sc ON sc.legislative_period = lp.legislative_period 
            AND sc.delegate_id = ds.id 
        WHERE 
            {filter}    
            AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
            AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
        GROUP BY 
            ds.name, m.party, sc.total_sessions_attended
        ORDER BY 
            {normalized} {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateCallToOrders>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
