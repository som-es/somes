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
pub struct DelegateAgeFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateAge {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    delegate_age: f64,
}

// #[debug_handler]
pub async fn age_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateAgeFilter>>,
) -> Result<Json<Vec<DelegateAge>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("ds.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Some("nr").with_sql_column("ds.council");
    let filter_arg4 = Manual("birthdate is not null").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3, filter_arg4];

    let filter = build_filter(&filters);

    let query = format!(
        "
        SELECT DISTINCT 
            ds.name AS delegate_name,
            ds.party AS delegate_party,
            ds.gender AS delegate_gender,
            (EXTRACT(YEAR FROM AGE(birthdate)) + 
            (EXTRACT(MONTH FROM AGE(birthdate)) / 12.0) + 
            (EXTRACT(DAY FROM AGE(birthdate)) / 365.25))::FLOAT as delegate_age
        FROM 
            delegates ds
        JOIN 
            plenar_speeches ps ON ps.delegate_id = ds.id
        JOIN 
            debates db ON db.id = ps.debate_id
        JOIN 
            plenar_infos pf ON pf.id = db.plenar_id
        WHERE 
            {filter}
ORDER BY 
    delegate_age DESC;
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateAge>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
