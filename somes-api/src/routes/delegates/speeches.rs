use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use somes_common_lib::DelegateByIdAndPage;
use utoipa::ToSchema;
use {combx::models::DbSpeechWithLink, combx::with_data::delegates::extract_delegate_speeches};

use crate::{routes::DelegateError, PgPoolConnection};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct SpeechesWithMaxPage {
    pub speeches: Vec<DbSpeechWithLink>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn speeches_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<SpeechesWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = crate::SPEECHES_PER_PAGE.parse().unwrap_or(16);
    Ok(
        extract_delegate_speeches(delegate_id, page, page_elements, &pg)
            .await
            .map(|(all_speeches_count, speeches)| SpeechesWithMaxPage {
                entry_count: all_speeches_count as i64,
                speeches,
                max_page: (all_speeches_count as f64 / page_elements as f64).ceil() as i64,
            })
            .map(Json)?,
    )
}
