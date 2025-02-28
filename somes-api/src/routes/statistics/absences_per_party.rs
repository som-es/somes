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
pub struct PartyAbsencesFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartyAbsences {
    party: String,
    party_members: i64,
    total_absences: i64,
    normalized_absences: f64,
}

// #[debug_handler]
pub async fn absences_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartyAbsencesFilter>>,
) -> Result<Json<Vec<PartyAbsences>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
     WITH party_member_counts AS ( 
    SELECT 
        ds.party, 
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
     m.party AS party,
     pmc.total_party_member_count AS party_members,
     COUNT(DISTINCT ab.id) AS total_absences,
     COUNT(DISTINCT ab.id)::FLOAT / pmc.total_party_member_count::FLOAT AS normalized_absences
FROM 
    absences ab
JOIN 
    delegates ds ON ab.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    plenar_infos pf ON pf.id = ab.plenary_session_id
JOIN
    party_member_counts pmc ON ds.party = pmc.party
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
GROUP BY 
    m.party, pmc.total_party_member_count
ORDER BY 
    total_absences {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartyAbsences>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
