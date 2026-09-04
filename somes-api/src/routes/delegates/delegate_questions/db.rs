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

enum PublicQuestionFilter {
    All,
    Delegate(i32),
    Id(i64),
}

struct PublicQuestionRow {
    id: i64,
    delegate_id: i32,
    subject: String,
    question_body: String,
    created_at: DateTime<Utc>,
    answer_body: Option<String>,
    answer_received_at: Option<DateTime<Utc>>,
}

pub(crate) async fn fetch_public_questions(
    pg: &sqlx::PgPool,
    delegate_id: Option<i32>,
    language: Language,
) -> Result<Vec<PublicDelegateQuestion>, GenericError> {
    let filter = match delegate_id {
        Some(delegate_id) => PublicQuestionFilter::Delegate(delegate_id),
        None => PublicQuestionFilter::All,
    };

    fetch_public_questions_with_filter(pg, filter, language).await
}

pub(crate) async fn find_public_question(
    pg: &sqlx::PgPool,
    question_id: i64,
    language: Language,
) -> Result<PublicDelegateQuestion, GenericError> {
    let mut questions =
        fetch_public_questions_with_filter(pg, PublicQuestionFilter::Id(question_id), language)
            .await?;

    questions.pop().ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Question was not found",
    )))
}

async fn fetch_public_questions_with_filter(
    pg: &sqlx::PgPool,
    filter: PublicQuestionFilter,
    language: Language,
) -> Result<Vec<PublicDelegateQuestion>, GenericError> {
    let mut questions = to_public_questions(fetch_public_question_rows(pg, filter).await?);
    attach_question_topics(pg, &mut questions, language).await?;

    Ok(questions)
}

async fn fetch_public_question_rows(
    pg: &sqlx::PgPool,
    filter: PublicQuestionFilter,
) -> Result<Vec<PublicQuestionRow>, GenericError> {
    let (delegate_id, question_id) = match filter {
        PublicQuestionFilter::All => (None, None),
        PublicQuestionFilter::Delegate(delegate_id) => (Some(delegate_id), None),
        PublicQuestionFilter::Id(question_id) => (None, Some(question_id)),
    };

    let rows = sqlx::query!(
        r#"
        SELECT
            q.id,
            q.delegate_id,
            q.subject,
            q.body AS question_body,
            q.created_at,
            a.body AS "answer_body?",
            a.received_at AS "answer_received_at?"
        FROM delegate_questions q
        LEFT JOIN delegate_question_answers a ON a.question_id = q.id
        WHERE ($1::INTEGER IS NULL OR q.delegate_id = $1)
            AND ($2::BIGINT IS NULL OR q.id = $2)
            AND q.status IN ('sent', 'answered')
        ORDER BY q.created_at DESC, a.received_at ASC NULLS LAST
        "#,
        delegate_id,
        question_id
    )
    .fetch_all(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(rows
        .into_iter()
        .map(|row| PublicQuestionRow {
            id: row.id,
            delegate_id: row.delegate_id,
            subject: row.subject,
            question_body: row.question_body,
            created_at: row.created_at,
            answer_body: row.answer_body,
            answer_received_at: row.answer_received_at,
        })
        .collect())
}

fn to_public_questions(rows: Vec<PublicQuestionRow>) -> Vec<PublicDelegateQuestion> {
    let mut questions: Vec<PublicDelegateQuestion> = Vec::new();

    for row in rows {
        if !matches!(questions.last(), Some(question) if question.id == row.id) {
            questions.push(PublicDelegateQuestion {
                id: row.id,
                delegate_id: row.delegate_id,
                subject: row.subject,
                body: row.question_body,
                created_at: row.created_at,
                topics: Vec::new(),
                answers: Vec::new(),
                created_at_date: row.created_at.naive_local().date(),
            });
        }

        if let (Some(body), Some(received_at)) = (row.answer_body, row.answer_received_at) {
            if let Some(question) = questions.last_mut() {
                question
                    .answers
                    .push(PublicDelegateQuestionAnswer { body, received_at });
            }
        }
    }

    questions
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

    attach_question_topics(pg, &mut questions, language).await?;

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

    let mut questions = vec![AdminDelegateQuestion {
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
    }];

    attach_question_topics(pg, &mut questions, language).await?;

    Ok(questions.remove(0))
}

trait WithTopics {
    fn question_id(&self) -> i64;
    fn set_topics(&mut self, topics: Vec<DelegateQuestionTopic>);
}

impl WithTopics for PublicDelegateQuestion {
    fn question_id(&self) -> i64 {
        self.id
    }

    fn set_topics(&mut self, topics: Vec<DelegateQuestionTopic>) {
        self.topics = topics;
    }
}

impl WithTopics for AdminDelegateQuestion {
    fn question_id(&self) -> i64 {
        self.id
    }

    fn set_topics(&mut self, topics: Vec<DelegateQuestionTopic>) {
        self.topics = topics;
    }
}

async fn attach_question_topics<T: WithTopics>(
    pg: &sqlx::PgPool,
    questions: &mut [T],
    language: Language,
) -> Result<(), GenericError> {
    let question_ids: Vec<i64> = questions.iter().map(WithTopics::question_id).collect();
    let mut topics = fetch_question_topics(pg, &question_ids, language).await?;

    for question in questions.iter_mut() {
        question.set_topics(topics.remove(&question.question_id()).unwrap_or_default());
    }

    Ok(())
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
