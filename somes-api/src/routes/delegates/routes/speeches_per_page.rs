use axum::{Json, extract::Query};
use combx::{DbInterjection, DbSpeechAiSummary, DbSpeechRelations, DbSpeechWithLink, FullSpeech};
use serde::{Deserialize, Serialize};
use somes_common_lib::DelegateByIdAndPage;
use utoipa::ToSchema;

use crate::{PgPoolConnection, routes::DelegateError};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct SpeechesWithMaxPage {
    pub speeches: Vec<FullSpeech>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn speeches_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<SpeechesWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;
    let page_elements: i64 = crate::SPEECHES_PER_PAGE.parse().unwrap_or(16);

    let entry_count = sqlx::query!(
        "select COUNT(*) from plenar_speeches where delegate_id = $1",
        delegate_id
    )
    .fetch_one(&pg)
    .await?
    .count
    .unwrap_or_default();

    let speeches = sqlx::query_as!(
        FullSpeech,
        r#"
        SELECT
            fs.id AS "id!",
            fs.debate_id AS "debate_id!",
            fs.delegate_id AS "delegate_id!",
            fs."speech: DbSpeechWithLink" AS "speech!: DbSpeechWithLink",
            fs."ai_summary: DbSpeechAiSummary" AS "ai_summary: DbSpeechAiSummary",
            fs."relations: Vec<DbSpeechRelations>" AS "relations!: Vec<DbSpeechRelations>",
            fs."received_interjections: Vec<DbInterjection>" AS "received_interjections!: Vec<DbInterjection>"
        FROM full_speeches fs
        INNER JOIN debates ON debates.id = fs.debate_id
        INNER JOIN plenar_infos pi ON pi.id = debates.plenar_id
        WHERE fs.delegate_id = $1
        ORDER BY pi.raw_data_created_at DESC, fs.id
        OFFSET $2 LIMIT $3
        "#,
        delegate_id,
        page * page_elements,
        page_elements
    )
    .fetch_all(&pg)
    .await?;

    Ok(Json(SpeechesWithMaxPage {
        speeches,
        entry_count,
        max_page: (entry_count as f64 / page_elements as f64).ceil() as i64,
    }))
}

/*
SELECT
            fs.id AS "id!",
            fs.debate_id AS "debate_id!",
            fs.delegate_id AS "delegate_id!",
            fs."speech: DbSpeechWithLink" AS "speech!: DbSpeechWithLink",
            fs."ai_summary: DbSpeechAiSummary" AS "ai_summary: DbSpeechAiSummary",
            fs."relations: Vec<DbSpeechRelations>" AS "relations!: Vec<DbSpeechRelations>"
        FROM full_speeches fs
        INNER JOIN debates ON debates.id = fs.debate_id
        INNER JOIN plenar_infos pi ON pi.id = debates.plenar_id
        WHERE fs.delegate_id = 145
        ORDER BY pi.raw_data_created_at DESC, fs.id
        OFFSET 0 LIMIT 16
 */
