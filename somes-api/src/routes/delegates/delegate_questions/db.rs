use chrono::{DateTime, Utc};
use common_scrapes::language::Language;
use reqwest::StatusCode;
use sqlx::Transaction;
use std::collections::{HashMap, HashSet};

use crate::GenericError;

use super::models::{
    AdminDelegateQuestion, DelegateQuestionTopic, PublicDelegateQuestion,
    PublicDelegateQuestionAnswer, QuestionDelivery,
};

pub(crate) async fn fetch_public_questions(
    pg: &sqlx::PgPool,
    delegate_id: Option<i32>,
    language: Language,
) -> Result<Vec<PublicDelegateQuestion>, GenericError> {
    let rows = sqlx::query!(
        r#"
        SELECT
            q.id AS question_id,
            q.delegate_id,
            q.subject,
            q.body AS question_body,
            q.created_at,
            a.body AS "answer_body?",
            a.received_at AS "answer_received_at?"
        FROM delegate_questions q
        LEFT JOIN delegate_question_answers a ON a.question_id = q.id
        WHERE ($1::INTEGER IS NULL OR q.delegate_id = $1)
            AND q.status IN ('sent', 'answered')
        ORDER BY q.created_at DESC, a.received_at ASC NULLS LAST
        "#,
        delegate_id
    )
    .fetch_all(pg)
    .await
    .unwrap();
    // .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let mut questions = Vec::new();
    let mut question_ids = Vec::new();
    let mut current_question_id = None;

    for row in rows {
        if current_question_id != Some(row.question_id) {
            questions.push(PublicDelegateQuestion {
                delegate_id: row.delegate_id,
                subject: row.subject,
                body: row.question_body,
                created_at: row.created_at,
                topics: Vec::new(),
                answers: Vec::new(),
                id: row.question_id,
            });
            question_ids.push(row.question_id);
            current_question_id = Some(row.question_id);
        }

        let answer_body: Option<String> = row.answer_body;
        let answer_received_at: Option<DateTime<Utc>> = row.answer_received_at;

        if let (Some(body), Some(received_at)) = (answer_body, answer_received_at) {
            if let Some(question) = questions.last_mut() {
                question
                    .answers
                    .push(PublicDelegateQuestionAnswer { body, received_at });
            }
        }
    }

    let topics = fetch_question_topics(pg, &question_ids, language).await?;
    for (question, question_id) in questions.iter_mut().zip(question_ids) {
        question.topics = topics.get(&question_id).cloned().unwrap_or_default();
    }

    Ok(questions)
}

pub(super) async fn fetch_review_questions(
    pg: &sqlx::PgPool,
    language: Language,
) -> Result<Vec<AdminDelegateQuestion>, GenericError> {
    let rows = sqlx::query!(
        "
        SELECT
            q.id,
            q.user_id,
            q.delegate_id,
            d.name AS delegate_name,
            q.recipient_email,
            q.recipient_kind,
            q.recipient_name,
            q.subject,
            q.body,
            q.status,
            q.created_at
        FROM delegate_questions q
        JOIN delegates d ON d.id = q.delegate_id
        WHERE q.status IN ('pending', 'failed')
        ORDER BY q.created_at ASC
        ",
    )
    .fetch_all(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let mut questions: Vec<AdminDelegateQuestion> = rows
        .into_iter()
        .map(|row| AdminDelegateQuestion {
            id: row.id,
            user_id: row.user_id,
            delegate_id: row.delegate_id,
            delegate_name: row.delegate_name,
            recipient_email: row.recipient_email,
            recipient_kind: row.recipient_kind,
            recipient_name: row.recipient_name,
            subject: row.subject,
            body: row.body,
            status: row.status,
            created_at: row.created_at,
            topics: Vec::new(),
        })
        .collect();

    let question_ids: Vec<i64> = questions.iter().map(|question| question.id).collect();
    let topics = fetch_question_topics(pg, &question_ids, language).await?;
    for question in questions.iter_mut() {
        question.topics = topics.get(&question.id).cloned().unwrap_or_default();
    }

    Ok(questions)
}

pub(super) async fn find_admin_question(
    pg: &sqlx::PgPool,
    question_id: i64,
    language: Language,
) -> Result<AdminDelegateQuestion, GenericError> {
    let row = sqlx::query!(
        "
        SELECT
            q.id,
            q.user_id,
            q.delegate_id,
            d.name AS delegate_name,
            q.recipient_email,
            q.recipient_kind,
            q.recipient_name,
            q.subject,
            q.body,
            q.status,
            q.created_at
        FROM delegate_questions q
        JOIN delegates d ON d.id = q.delegate_id
        WHERE q.id = $1
        ",
        question_id
    )
    .fetch_optional(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?
    .ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Question was not found",
    )))?;

    let topics = fetch_question_topics(pg, &[question_id], language).await?;

    Ok(AdminDelegateQuestion {
        id: row.id,
        user_id: row.user_id,
        delegate_id: row.delegate_id,
        delegate_name: row.delegate_name,
        recipient_email: row.recipient_email,
        recipient_kind: row.recipient_kind,
        recipient_name: row.recipient_name,
        subject: row.subject,
        body: row.body,
        status: row.status,
        created_at: row.created_at,
        topics: topics.get(&question_id).cloned().unwrap_or_default(),
    })
}

pub(super) async fn fetch_question_topics(
    pg: &sqlx::PgPool,
    question_ids: &[i64],
    language: Language,
) -> Result<HashMap<i64, Vec<DelegateQuestionTopic>>, GenericError> {
    if question_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT ON (qt.question_id, qt.topic_id)
            qt.question_id,
            ut.id_as_hash::text AS "topic_id!",
            ut.topic_name AS topic
        FROM delegate_question_topics qt
        JOIN unique_eurovoc_topics ut ON ut.id_as_hash = qt.topic_id
        WHERE qt.question_id = ANY($1)
        ORDER BY
            qt.question_id,
            qt.topic_id,
            (ut.language = $2) DESC,
            (ut.language = 'de') DESC,
            ut.topic_name
        "#,
        question_ids,
        language.as_str()
    )
    .fetch_all(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let mut topics: HashMap<i64, Vec<DelegateQuestionTopic>> = HashMap::new();
    for row in rows {
        topics
            .entry(row.question_id)
            .or_default()
            .push(DelegateQuestionTopic {
                id: row.topic_id,
                topic: row.topic,
            });
    }

    Ok(topics)
}

pub(super) async fn create_question(
    pg: &sqlx::PgPool,
    user_id: i32,
    delegate_id: i32,
    recipient_email: &str,
    delivery: QuestionDelivery,
    recipient_name: &str,
    subject: &str,
    body: &str,
    outgoing_message_id: &str,
    topic_ids: &[String],
) -> Result<i64, GenericError> {
    let mut tx: Transaction<'_, sqlx::Postgres> = pg
        .begin()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let question_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO delegate_questions
            (user_id, delegate_id, recipient_email, recipient_kind, recipient_name, subject, body, outgoing_message_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        ",
        user_id,
        delegate_id,
        recipient_email,
        delivery.as_str(),
        recipient_name,
        subject,
        body,
        outgoing_message_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    link_question_topics(&mut tx, question_id, topic_ids).await?;

    tx.commit()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(question_id)
}

pub(super) async fn update_question(
    pg: &sqlx::PgPool,
    question_id: i64,
    subject: Option<&str>,
    body: Option<&str>,
    topic_ids: Option<&[String]>,
) -> Result<(), GenericError> {
    let mut tx: Transaction<'_, sqlx::Postgres> = pg
        .begin()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    if let Some(subject) = subject {
        sqlx::query!(
            "UPDATE delegate_questions SET subject = $2 WHERE id = $1",
            question_id,
            subject
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;
    }

    if let Some(body) = body {
        sqlx::query!(
            "UPDATE delegate_questions SET body = $2 WHERE id = $1",
            question_id,
            body
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;
    }

    if let Some(topic_ids) = topic_ids {
        sqlx::query!(
            "DELETE FROM delegate_question_topics WHERE question_id = $1",
            question_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

        link_question_topics(&mut tx, question_id, topic_ids).await?;
    }

    sqlx::query!(
        "UPDATE delegate_questions SET updated_at = NOW() WHERE id = $1",
        question_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    tx.commit()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(())
}

async fn link_question_topics(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    question_id: i64,
    topic_ids: &[String],
) -> Result<(), GenericError> {
    let mut linked = HashSet::new();

    for topic_id in topic_ids {
        let topic_id = topic_id
            .parse::<i64>()
            .map_err(|_| GenericError::Custom((StatusCode::BAD_REQUEST, "invalid topic id")))?;

        if !linked.insert(topic_id) {
            continue;
        }

        let known = sqlx::query_scalar!(
            r#"SELECT EXISTS (SELECT 1 FROM unique_eurovoc_topics WHERE id_as_hash = $1) AS "exists!""#,
            topic_id
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

        if !known {
            return Err(GenericError::Custom((
                StatusCode::BAD_REQUEST,
                "unknown topic id",
            )));
        }

        sqlx::query!(
            "insert into delegate_question_topics (question_id, topic_id) values ($1, $2)",
            question_id,
            topic_id
        )
        .execute(&mut **tx)
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;
    }

    Ok(())
}

pub(super) async fn set_question_status(
    pg: &sqlx::PgPool,
    question_id: i64,
    status: &str,
) -> Result<(), GenericError> {
    sqlx::query!(
        "
        UPDATE delegate_questions
        SET status = $1::text,
            sent_at = CASE WHEN $1::text = 'sent' THEN NOW() ELSE sent_at END,
            updated_at = NOW()
        WHERE id = $2
        ",
        status,
        question_id
    )
    .execute(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(())
}
