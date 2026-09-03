use axum::Json;
use serde::Serialize;
use somes_common_lib::TopicInfluence;

use crate::{GenericError, PgPoolConnection};

#[derive(Debug, Serialize)]
pub struct OrientationQuestionResponse {
    pub id: i32,
    pub question: String,
    pub is_left: Option<bool>,
    pub is_liberal: Option<bool>,
    pub is_part_of: Vec<String>,
    pub strong_reference_answers: Vec<StrongReferenceAnswer>,
    pub topics: Vec<String>,
    pub topics_influence: Vec<TopicInfluence>,
    pub detailed_topics: Vec<String>,
    pub detailed_topics_influence: Vec<TopicInfluence>,
}

#[derive(Debug, Serialize)]
pub struct StrongReferenceAnswer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String,
    pub stance_llm: String,
    pub is_strong_reference: Option<bool>,
    pub model_used: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub full_stance: Option<serde_json::Value>,
}

pub async fn orientation_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<OrientationQuestionResponse>>, GenericError> {
    // Fetch questions where is_part_of IS NOT NULL, ordered by id asc
    let questions = sqlx::query_as!(
        QuestionRow,
        r#"
        SELECT
            id,
            question,
            is_left,
            is_liberal,
            is_part_of
        FROM political_questions
        WHERE is_part_of IS NOT NULL
        ORDER BY id ASC
        "#
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    if questions.is_empty() {
        return Ok(Json(vec![]));
    }

    let question_ids: Vec<i32> = questions.iter().map(|q| q.id).collect();

    // Fetch strong reference answers
    let answers = sqlx::query_as!(
        StrongReferenceAnswer,
        r#"
        SELECT
            id,
            question_id,
            answer,
            stance_llm,
            is_strong_reference,
            model_used,
            created_at,
            full_stance
        FROM political_answers
        WHERE question_id = ANY($1) AND is_strong_reference = true
        "#,
        &question_ids
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    // Fetch topics
    let topics_rows = sqlx::query!(
        "SELECT question_id, topic FROM political_questions_topics WHERE question_id = ANY($1)",
        &question_ids
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    // Fetch topics influence
    let topics_influence_rows = sqlx::query!(
        "SELECT question_id, topic, influence FROM political_questions_topics_influence WHERE question_id = ANY($1)",
        &question_ids
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    // Fetch detailed topics
    let detailed_topics_rows = sqlx::query!(
        "SELECT question_id, topic FROM political_questions_detailed_topics WHERE question_id = ANY($1)",
        &question_ids
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    // Fetch detailed topics influence
    let detailed_topics_influence_rows = sqlx::query!(
        "SELECT question_id, topic, influence FROM political_questions_detailed_topics_influence WHERE question_id = ANY($1)",
        &question_ids
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    // Assemble maps
    use std::collections::HashMap;
    let mut answers_map: HashMap<i32, Vec<StrongReferenceAnswer>> = HashMap::new();
    for a in answers {
        answers_map.entry(a.question_id).or_default().push(a);
    }

    let mut topics_map: HashMap<i32, Vec<String>> = HashMap::new();
    for row in topics_rows {
        topics_map
            .entry(row.question_id)
            .or_default()
            .push(row.topic);
    }

    let mut topics_influence_map: HashMap<i32, Vec<TopicInfluence>> = HashMap::new();
    for row in topics_influence_rows {
        topics_influence_map
            .entry(row.question_id)
            .or_default()
            .push(TopicInfluence {
                topic: row.topic,
                influence: row.influence,
            });
    }

    let mut detailed_topics_map: HashMap<i32, Vec<String>> = HashMap::new();
    for row in detailed_topics_rows {
        detailed_topics_map
            .entry(row.question_id)
            .or_default()
            .push(row.topic);
    }

    let mut detailed_topics_influence_map: HashMap<i32, Vec<TopicInfluence>> = HashMap::new();
    for row in detailed_topics_influence_rows {
        detailed_topics_influence_map
            .entry(row.question_id)
            .or_default()
            .push(TopicInfluence {
                topic: row.topic,
                influence: row.influence,
            });
    }

    let responses = questions
        .into_iter()
        .map(|q| OrientationQuestionResponse {
            id: q.id,
            question: q.question,
            is_left: q.is_left,
            is_liberal: q.is_liberal,
            is_part_of: q.is_part_of.unwrap_or_default(),
            strong_reference_answers: answers_map.remove(&q.id).unwrap_or_default(),
            topics: topics_map.remove(&q.id).unwrap_or_default(),
            topics_influence: topics_influence_map.remove(&q.id).unwrap_or_default(),
            detailed_topics: detailed_topics_map.remove(&q.id).unwrap_or_default(),
            detailed_topics_influence: detailed_topics_influence_map
                .remove(&q.id)
                .unwrap_or_default(),
        })
        .collect();

    Ok(Json(responses))
}

// Helper row for questions
struct QuestionRow {
    id: i32,
    question: String,
    is_left: Option<bool>,
    is_liberal: Option<bool>,
    is_part_of: Option<Vec<String>>,
}

#[cfg(test)]
#[path = "orientation_questions/tests/orientation_questions.rs"]
mod tests;
