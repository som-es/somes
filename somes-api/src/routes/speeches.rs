use axum::{Json, Router, extract::Path, routing::get};
use combx::{DbInterjection, DbSpeechAiSummary, DbSpeechRelations, DbSpeechWithLink, FullSpeech};
use somes_common_lib::SpeechById;

use crate::{AppState, GenericError, PgPoolConnection};

pub fn create_speeches_router() -> Router<AppState> {
    Router::new().route("/{speech_id}", get(speech_by_id_route))
}

pub async fn speech_by_id_route(
    PgPoolConnection(pg): PgPoolConnection,
    Path(speech_id): Path<SpeechById>,
) -> crate::Result<Json<Option<FullSpeech>>> {
    sqlx::query_as!(
        FullSpeech,
        r#"SELECT
            fs.id AS "id!",
            fs.debate_id AS "debate_id!",
            fs.delegate_id AS "delegate_id!",
            fs."speech: DbSpeechWithLink" AS "speech!: DbSpeechWithLink",
            fs."ai_summary: DbSpeechAiSummary" AS "ai_summary: DbSpeechAiSummary",
            fs."relations: Vec<DbSpeechRelations>" AS "relations!: Vec<DbSpeechRelations>",
            fs."received_interjections: Vec<DbInterjection>" AS "received_interjections!: Vec<DbInterjection>"
        FROM full_speeches fs
        WHERE id = $1
        "#,
        speech_id.speech_id
    )
    .fetch_optional(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))
    .map(Json)
}
