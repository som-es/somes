use axum::{Json, extract::Query};
use combx::{DbAiSummary, OptionalDecree};
use somes_common_lib::{Days, Document};
use sqlx::{PgPool, query_as};

use crate::{PgPoolConnection, routes::FilterError};

pub async fn latest_decrees_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(days): Query<Days>,
) -> Result<Json<Vec<OptionalDecree>>, FilterError> {
    if days.days > 180 {
        return Err(FilterError::InvalidDays(days.days as u32));
    }

    Ok(extract_latest_ministrial_decrees(&pg, days.days as i32)
        .await
        .map(Json)?)
}

pub async fn extract_latest_ministrial_decrees(
    pg: &PgPool,
    days: i32,
) -> sqlx::Result<Vec<OptionalDecree>> {
    let decrees = query_as!(
        OptionalDecree,
        "
        select * from ministrial_decrees_with_docs
        where publication_date > NOW() - make_interval(days => $1)
    order by publication_date desc",
        days
    )
    .fetch_all(pg)
    .await?;
    Ok(decrees)
}
