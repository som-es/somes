use axum::Json;
use common_scrapes::language::Language;
use sqlx::query_as;

use crate::{GenericError, ParliamentCtx, PgPoolConnection};

use super::UniqueTopic;

pub async fn eurovoc_topics_route(
    PgPoolConnection(pg): PgPoolConnection,
    ParliamentCtx(parliament): ParliamentCtx,
) -> Result<Json<Vec<UniqueTopic>>, GenericError> {
    let lang = match parliament {
        combx::Parliament::At => Language::De,
        combx::Parliament::Eu => Language::En,
    };
    query_as!(
        UniqueTopic,
        r#"select id_as_hash::text as "id!", topic_name as topic from unique_eurovoc_topics where language = $1 order by topic"#, lang.as_str()
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}

pub async fn topics_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, GenericError> {
    query_as!(
        UniqueTopic,
        r#"select id::text as "id!", topic_name as topic from unique_topics order by topic"#
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}
