mod mail;
mod models;

use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use delegate_question_mail::new_question_message_id;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::{Row, Transaction};
use std::collections::HashMap;

use crate::{AppState, GenericError, PgPoolConnection, jwt::Claims};

use self::{
    mail::send_question_mail,
    models::{DelegateContact, PublicDelegateQuestionAnswer, QuestionDelivery},
};

pub use models::{
    AdminDelegateQuestion, CreateDelegateQuestion, DelegateQuestionCreated,
    DelegateQuestionRecipient, PublicDelegateQuestion,
};

const MAX_SUBJECT_LENGTH: usize = 255;
const MAX_BODY_LENGTH: usize = 10_000;
const PARTY_QUESTION_RECIPIENTS: &str =
    include_str!("../../../../config/party-question-recipients.json");

static PARTY_RECIPIENTS: Lazy<HashMap<String, PartyRecipientConfig>> = Lazy::new(|| {
    serde_json::from_str(PARTY_QUESTION_RECIPIENTS)
        .expect("party question recipient configuration must be valid JSON")
});

#[derive(Debug, Deserialize)]
struct PartyRecipientConfig {
    name: String,
    email: String,
}

pub fn create_delegate_questions_router() -> Router<AppState> {
    Router::new()
        .route("/", get(all_delegate_questions_route))
        .route(
            "/delegate/{delegate_id}",
            get(delegate_questions_route).post(ask_delegate_question_route),
        )
        .route(
            "/delegate/{delegate_id}/question_recipient",
            get(delegate_question_recipient_route),
        )
        .route("/pending", get(pending_delegate_questions_route))
        .route(
            "/{question_id}/approve",
            post(approve_delegate_question_route),
        )
        .route(
            "/{question_id}/reject",
            post(reject_delegate_question_route),
        )
}

pub async fn ask_delegate_question_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path(delegate_id): Path<i32>,
    Json(question): Json<CreateDelegateQuestion>,
) -> Result<Json<DelegateQuestionCreated>, GenericError> {
    let subject = question.subject.trim().to_owned();
    let body = question.body.trim().to_owned();
    validate_question(&subject, &body)?;

    let delegate = find_delegate_contact(&pg, delegate_id).await?;
    let outgoing_message_id = new_question_message_id();

    let question_id = create_question(
        &pg,
        claims.id,
        delegate_id,
        &delegate.recipient_email,
        delegate.delivery,
        &delegate.recipient_name,
        &subject,
        &body,
        &outgoing_message_id,
    )
    .await?;

    Ok(Json(DelegateQuestionCreated {
        id: question_id,
        delivery: delegate.delivery,
        recipient_name: delegate.recipient_name,
        status: "pending".to_string(),
    }))
}

pub async fn delegate_question_recipient_route(
    PgPoolConnection(pg): PgPoolConnection,
    Path(delegate_id): Path<i32>,
) -> Result<Json<DelegateQuestionRecipient>, GenericError> {
    let recipient = find_delegate_contact(&pg, delegate_id).await?;

    Ok(Json(DelegateQuestionRecipient {
        delivery: recipient.delivery,
        recipient_name: recipient.recipient_name,
    }))
}

pub async fn delegate_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
    Path(delegate_id): Path<i32>,
) -> Result<Json<Vec<PublicDelegateQuestion>>, GenericError> {
    fetch_public_questions(&pg, Some(delegate_id))
        .await
        .map(Json)
}

pub async fn all_delegate_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<PublicDelegateQuestion>>, GenericError> {
    fetch_public_questions(&pg, None).await.map(Json)
}

pub async fn pending_delegate_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<Vec<AdminDelegateQuestion>>, GenericError> {
    ensure_admin(&claims)?;
    fetch_review_questions(&pg).await.map(Json)
}

pub async fn approve_delegate_question_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path(question_id): Path<i64>,
) -> Result<Json<AdminDelegateQuestion>, GenericError> {
    ensure_admin(&claims)?;

    let question = find_admin_question(&pg, question_id).await?;
    if question.status != "pending" && question.status != "failed" {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be approved",
        )));
    }

    send_question_mail(&pg, question_id).await?;
    find_admin_question(&pg, question_id).await.map(Json)
}

pub async fn reject_delegate_question_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path(question_id): Path<i64>,
) -> Result<Json<AdminDelegateQuestion>, GenericError> {
    ensure_admin(&claims)?;
    let question = find_admin_question(&pg, question_id).await?;
    if question.status != "pending" && question.status != "failed" {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be rejected",
        )));
    }

    set_question_status(&pg, question_id, "rejected").await?;
    find_admin_question(&pg, question_id).await.map(Json)
}

fn ensure_admin(claims: &Claims) -> Result<(), GenericError> {
    if claims.is_admin {
        return Ok(());
    }

    Err(GenericError::Custom((
        StatusCode::UNAUTHORIZED,
        "insufficient permissions",
    )))
}

fn validate_question(subject: &str, body: &str) -> Result<(), GenericError> {
    if subject.is_empty() || body.is_empty() {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "Subject and body are required",
        )));
    }

    if subject.chars().count() > MAX_SUBJECT_LENGTH || body.chars().count() > MAX_BODY_LENGTH {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "Question is too long",
        )));
    }

    Ok(())
}

async fn find_delegate_contact(
    pg: &sqlx::PgPool,
    delegate_id: i32,
) -> Result<DelegateContact, GenericError> {
    let row = sqlx::query(
        "
        SELECT d.name, d.party, c.mail
        FROM delegates d
        JOIN contacts c ON c.id = d.id
        WHERE d.id = $1
        ",
    )
    .bind(delegate_id)
    .fetch_optional(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?
    .ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Delegate was not found",
    )))?;

    let email: Option<String> = row
        .try_get("mail")
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let name: String = row
        .try_get("name")
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    if let Some(email) = email.filter(|email| !email.trim().is_empty()) {
        return Ok(DelegateContact {
            recipient_name: name.clone(),
            name,
            recipient_email: email,
            delivery: QuestionDelivery::Delegate,
        });
    }

    let party: Option<String> = row
        .try_get("party")
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;
    let party = party.ok_or(GenericError::Custom((
        StatusCode::UNPROCESSABLE_ENTITY,
        "Delegate has no email address or party assignment",
    )))?;
    let recipient = PARTY_RECIPIENTS
        .get(party.trim())
        .ok_or(GenericError::Custom((
            StatusCode::UNPROCESSABLE_ENTITY,
            "No question recipient is configured for this party",
        )))?;

    Ok(DelegateContact {
        name,
        recipient_name: recipient.name.clone(),
        recipient_email: recipient.email.clone(),
        delivery: QuestionDelivery::Party,
    })
}

async fn fetch_public_questions(
    pg: &sqlx::PgPool,
    delegate_id: Option<i32>,
) -> Result<Vec<PublicDelegateQuestion>, GenericError> {
    let rows = sqlx::query(
        "
        SELECT
            q.id AS question_id,
            q.delegate_id,
            q.subject,
            q.body AS question_body,
            q.created_at,
            a.body AS answer_body,
            a.received_at AS answer_received_at
        FROM delegate_questions q
        LEFT JOIN delegate_question_answers a ON a.question_id = q.id
        WHERE ($1::INTEGER IS NULL OR q.delegate_id = $1)
            AND q.status IN ('sent', 'answered')
        ORDER BY q.created_at DESC, a.received_at ASC NULLS LAST
        ",
    )
    .bind(delegate_id)
    .fetch_all(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let mut questions = Vec::new();
    let mut current_question_id = None;

    for row in rows {
        let question_id: i64 = row
            .try_get("question_id")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?;

        if current_question_id != Some(question_id) {
            questions.push(PublicDelegateQuestion {
                delegate_id: row
                    .try_get("delegate_id")
                    .map_err(|error| GenericError::SqlFailure(Some(error)))?,
                subject: row
                    .try_get("subject")
                    .map_err(|error| GenericError::SqlFailure(Some(error)))?,
                body: row
                    .try_get("question_body")
                    .map_err(|error| GenericError::SqlFailure(Some(error)))?,
                created_at: row
                    .try_get("created_at")
                    .map_err(|error| GenericError::SqlFailure(Some(error)))?,
                answers: Vec::new(),
            });
            current_question_id = Some(question_id);
        }

        let answer_body: Option<String> = row
            .try_get("answer_body")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?;
        let answer_received_at: Option<DateTime<Utc>> = row
            .try_get("answer_received_at")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?;

        if let (Some(body), Some(received_at)) = (answer_body, answer_received_at) {
            if let Some(question) = questions.last_mut() {
                question
                    .answers
                    .push(PublicDelegateQuestionAnswer { body, received_at });
            }
        }
    }

    Ok(questions)
}

async fn fetch_review_questions(
    pg: &sqlx::PgPool,
) -> Result<Vec<AdminDelegateQuestion>, GenericError> {
    let rows = sqlx::query(
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

    rows.into_iter().map(admin_question_from_row).collect()
}

async fn find_admin_question(
    pg: &sqlx::PgPool,
    question_id: i64,
) -> Result<AdminDelegateQuestion, GenericError> {
    let row = sqlx::query(
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
    )
    .bind(question_id)
    .fetch_optional(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?
    .ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Question was not found",
    )))?;

    admin_question_from_row(row)
}

fn admin_question_from_row(
    row: sqlx::postgres::PgRow,
) -> Result<AdminDelegateQuestion, GenericError> {
    Ok(AdminDelegateQuestion {
        id: row
            .try_get("id")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        user_id: row
            .try_get("user_id")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        delegate_id: row
            .try_get("delegate_id")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        delegate_name: row
            .try_get("delegate_name")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        recipient_email: row
            .try_get("recipient_email")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        recipient_kind: row
            .try_get("recipient_kind")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        recipient_name: row
            .try_get("recipient_name")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        subject: row
            .try_get("subject")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        body: row
            .try_get("body")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        status: row
            .try_get("status")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
        created_at: row
            .try_get("created_at")
            .map_err(|error| GenericError::SqlFailure(Some(error)))?,
    })
}

async fn create_question(
    pg: &sqlx::PgPool,
    user_id: i32,
    delegate_id: i32,
    recipient_email: &str,
    delivery: QuestionDelivery,
    recipient_name: &str,
    subject: &str,
    body: &str,
    outgoing_message_id: &str,
) -> Result<i64, GenericError> {
    let mut transaction: Transaction<'_, sqlx::Postgres> = pg
        .begin()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    let question_id: i64 = sqlx::query_scalar(
        "
        INSERT INTO delegate_questions
            (user_id, delegate_id, recipient_email, recipient_kind, recipient_name, subject, body, outgoing_message_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        ",
    )
    .bind(user_id)
    .bind(delegate_id)
    .bind(recipient_email)
    .bind(delivery.as_str())
    .bind(recipient_name)
    .bind(subject)
    .bind(body)
    .bind(outgoing_message_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    transaction
        .commit()
        .await
        .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(question_id)
}

async fn set_question_status(
    pg: &sqlx::PgPool,
    question_id: i64,
    status: &str,
) -> Result<(), GenericError> {
    sqlx::query(
        "
        UPDATE delegate_questions
        SET status = $1,
            sent_at = CASE WHEN $1 = 'sent' THEN NOW() ELSE sent_at END,
            updated_at = NOW()
        WHERE id = $2
        ",
    )
    .bind(status)
    .bind(question_id)
    .execute(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    Ok(())
}
