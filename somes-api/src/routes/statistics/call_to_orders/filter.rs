use axum::Json;

use sqlx::{prelude::FromRow, Postgres};

use crate::{
    routes::statistics::{
        error::StatisticsResponse,
        filtering::{bind_values, build_filter, IntoFilterArgument},
    },
    PgPoolConnection,
};

#[derive(Default)]
pub struct CallToOrderFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
}

#[derive(PartialEq, Debug, Clone, FromRow)]
pub struct CallToOrdersForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    total_order_calls: i64,
}

pub async fn call_to_order_function(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    // irgendwie alle fields zu so ding da machen (array)
    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.gender.with_sql_column("ds.gender");
    let filter_arg2 = filter.party.with_sql_column("ds.party");
    let filter_arg3 = Some("nr").with_sql_column("council");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    // daraus kriegst du dann einen string der den filter in der query represented
    let filter = build_filter(&filters);

    let query = format!(
        "
    SELECT 
        ds.name AS delegate_name,
        ds.party AS delegate_party,
        ds.gender AS delegate_gender,
        COUNT(cto.id) AS total_order_calls
    FROM 
        call_to_order cto
    JOIN 
        delegates ds ON cto.receiver_id = ds.id
    JOIN 
        plenar_infos pf ON pf.id = cto.plenar_id
    WHERE 
        {filter}
    GROUP BY 
        ds.id, ds.name, ds.party, ds.gender
    ORDER BY 
        total_order_calls DESC;

"
    );

    let mut filtered_query = sqlx::query_as::<Postgres, CallToOrdersForDelegate>(&query);
    // setzt dann die filter werte auf die query parameter ok??
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
