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
pub struct PartyCallToOrdersFilter {
    legis_period: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartyCallToOrders {
    delegate_party: String,
    party_members: i64,
    total_order_calls: i64,
    normalized_calls_to_order: f64,
}

// #[debug_handler]
pub async fn call_to_orders_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartyCallToOrdersFilter>>,
) -> Result<Json<Vec<PartyCallToOrders>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr ").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized { "normalized_calls_to_order" } else { "total_order_calls" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        WITH party_member_counts AS (
    SELECT 
        ds.party AS party, 
        COUNT(DISTINCT ds.id) AS total_party_member_count
    FROM delegates ds
    JOIN mandates m ON m.delegate_id = ds.id
    JOIN plenar_infos pf ON 1=1  
    WHERE {filter}
        AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
        AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
    GROUP BY ds.party
)
        SELECT 
            m.party AS delegate_party,
            pmc.total_party_member_count AS party_members,
            COUNT(cto.id) AS total_order_calls,
            COUNT(cto.id)::FLOAT / pmc.total_party_member_count::FLOAT AS normalized_calls_to_order
        FROM 
            call_to_order cto
        JOIN 
            delegates ds ON cto.receiver_id = ds.id
        JOIN 
            plenar_infos pf ON pf.id = cto.plenar_id
        JOIN 
            mandates m ON m.delegate_id = ds.id
        JOIN
            party_member_counts pmc ON ds.party = pmc.party
        WHERE 
            {filter}    
            AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = cto.plenar_id)
            AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = cto.plenar_id))
        GROUP BY 
            m.party, pmc.total_party_member_count
        ORDER BY 
            {normalized} {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartyCallToOrders>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
