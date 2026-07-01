use axum::{extract::Path, Json};
use chrono::{DateTime, Utc};
use delegate_question_mail::new_question_message_id;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Row, Transaction};
use std::collections::HashMap;

use crate::{
    email::{send_mail_with_message_id, MAILER},
    jwt::Claims,
    GenericError, PgPoolConnection,
};

const MAX_SUBJECT_LENGTH: usize = 255;
const MAX_BODY_LENGTH: usize = 10_000;
const DELEGATE_QUESTION_TEMPLATE: &str =
    include_str!("../../email/delegate_question_template.html");
const PARTY_QUESTION_TEMPLATE: &str = include_str!("../../email/party_question_template.html");
const PARTY_QUESTION_RECIPIENTS: &str =
    include_str!("../../../config/party-question-recipients.json");

static PARTY_RECIPIENTS: Lazy<HashMap<String, PartyRecipientConfig>> = Lazy::new(|| {
    serde_json::from_str(PARTY_QUESTION_RECIPIENTS)
        .expect("party question recipient configuration must be valid JSON")
});

#[derive(Debug, Deserialize)]
pub struct CreateDelegateQuestion {
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct DelegateQuestionCreated {
    pub id: i64,
    pub delivery: QuestionDelivery,
    pub recipient_name: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum QuestionDelivery {
    Delegate,
    Party,
}

impl QuestionDelivery {
    fn as_str(self) -> &'static str {
        match self {
            Self::Delegate => "delegate",
            Self::Party => "party",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DelegateQuestionRecipient {
    pub delivery: QuestionDelivery,
    pub recipient_name: String,
}

#[derive(Debug, Serialize)]
pub struct PublicDelegateQuestion {
    pub delegate_id: i32,
    pub subject: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub answers: Vec<PublicDelegateQuestionAnswer>,
}

#[derive(Debug, Serialize)]
pub struct PublicDelegateQuestionAnswer {
    pub body: String,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct PartyRecipientConfig {
    name: String,
    email: String,
}

struct DelegateContact {
    name: String,
    recipient_name: String,
    recipient_email: String,
    delivery: QuestionDelivery,
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

    let mail_subject = format!("Neue Frage über somes.at: {subject}");
    let mail_content = render_question_mail(&delegate, &subject, &body);
    let recipient_email = delegate.recipient_email.clone();

    let mail_result = tokio::task::spawn_blocking(move || {
        send_mail_with_message_id(
            &MAILER,
            &recipient_email,
            &mail_subject,
            mail_content,
            Some(outgoing_message_id),
        )
        .map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| {
        log::error!("delegate question mail task failed: {error}");
        GenericError::Custom((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not send question email",
        ))
    })?;

    match mail_result {
        Ok(()) => set_question_status(&pg, question_id, "sent").await?,
        Err(error) => {
            log::error!("sending delegate question {question_id} failed: {error}");
            set_question_status(&pg, question_id, "failed").await?;
            return Err(GenericError::Custom((
                StatusCode::BAD_GATEWAY,
                "Could not send question email",
            )));
        }
    }

    Ok(Json(DelegateQuestionCreated {
        id: question_id,
        delivery: delegate.delivery,
        recipient_name: delegate.recipient_name,
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

fn render_question_mail(recipient: &DelegateContact, subject: &str, body: &str) -> String {
    let template = match recipient.delivery {
        QuestionDelivery::Delegate => DELEGATE_QUESTION_TEMPLATE,
        QuestionDelivery::Party => PARTY_QUESTION_TEMPLATE,
    };

    template
        .replace("{*DELEGATE_NAME*}", &escape_html(&recipient.name))
        .replace("{*PARTY_NAME*}", &escape_html(&recipient.recipient_name))
        .replace("{*QUESTION_SUBJECT*}", &escape_html(subject))
        .replace(
            "{*QUESTION_BODY*}",
            &escape_html(body).replace('\n', "<br>"),
        )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::{render_question_mail, DelegateContact, QuestionDelivery};

    #[test]
    fn renders_question_as_safe_html() {
        let rendered = render_question_mail(
            &DelegateContact {
                name: "Max <Mustermann>".to_string(),
                recipient_name: "Max <Mustermann>".to_string(),
                recipient_email: "max@example.com".to_string(),
                delivery: QuestionDelivery::Delegate,
            },
            "Ist <b>das</b> geplant?",
            "Bitte <script>alert('x')</script>",
        );

        assert!(rendered.contains("Max &lt;Mustermann&gt;"));
        assert!(rendered.contains("&lt;script&gt;alert(&#x27;x&#x27;)&lt;/script&gt;"));
        assert!(!rendered.contains("somes-q:"));
    }

    #[test]
    fn renders_party_fallback_mail() {
        let rendered = render_question_mail(
            &DelegateContact {
                name: "Max Mustermann".to_string(),
                recipient_name: "ÖVP-Klub".to_string(),
                recipient_email: "team@example.com".to_string(),
                delivery: QuestionDelivery::Party,
            },
            "Frage",
            "Bitte um Antwort",
        );

        assert!(rendered.contains("Sehr geehrtes Team des ÖVP-Klub"));
        assert!(rendered.contains("Max Mustermann"));
    }
}
