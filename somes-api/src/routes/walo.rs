mod error;

use axum::Json;
use error::WaloErrorResponse;
use serde::{Deserialize, Serialize};

use crate::PgPoolConnection;

#[derive(Clone, Serialize, Deserialize)]
pub struct WaloQuestion {
    id: i32,
    question_statement: Option<String>,
    new_keywords_topics: Option<String>,
    spoe_justification: Option<String>,
    gruene_justification: Option<String>,
    neos_justification: Option<String>,
    fpoe_justification: Option<String>,
    oevp_justification: Option<String>,
    somes_link: Option<String>,
    law_link: Option<String>,
    erklaerbaer: Option<String>,
}

pub async fn walo_questions(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<WaloQuestion>>, WaloErrorResponse> {
    sqlx::query_as!(
        WaloQuestion,
        "
        select * from walo
    "
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| WaloErrorResponse::QuestionResponseError)
}
