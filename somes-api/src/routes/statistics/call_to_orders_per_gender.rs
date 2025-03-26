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
pub struct GenderCallToOrdersFilter {
    legis_period: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenderCallToOrders {
    delegate_gender: String,
    gender_members: i64,
    total_order_calls: i64,
    normalized_calls_to_order: f64,
}

// #[debug_handler]
pub async fn call_to_orders_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<GenderCallToOrdersFilter>>,
) -> Result<Json<Vec<GenderCallToOrders>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_calls_to_order"
    } else {
        "total_order_calls"
    };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        WITH gender_counts AS (
    SELECT 
        ds.gender, 
        COUNT(DISTINCT ds.id) AS total_gender_count
    FROM delegates ds
    JOIN mandates m ON m.delegate_id = ds.id
    JOIN plenar_infos pf ON 1=1  
    WHERE {filter}
        AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
        AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
    GROUP BY ds.gender
)
       SELECT 
            ds.gender AS delegate_gender,
            gc.total_gender_count AS gender_members,
            COUNT(cto.id) AS total_order_calls,
            COUNT(cto.id)::FLOAT / gc.total_gender_count::FLOAT AS normalized_calls_to_order
        FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN 
            gender_counts gc ON ds.gender = gc.gender
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
GROUP BY 
    ds.gender, gc.total_gender_count
ORDER BY 
    {normalized} {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, GenderCallToOrders>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
