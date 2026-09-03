mod search;

use axum::{
    Json,
    extract::{Path, Query},
};
use combx::{Index, Parliament};
use common_scrapes::language::Language;
use delegate_question_mail::new_question_message_id;
use reqwest::StatusCode;
pub(super) use search::*;

pub use super::models::{
    AdminDelegateQuestion, CreateDelegateQuestion, DelegateQuestionCreated, DelegateQuestionQuery,
    DelegateQuestionRecipient, PublicDelegateQuestion, UpdateDelegateQuestion,
};
use crate::{
    GenericError, ParliamentCtx, PgPoolConnection,
    jwt::Claims,
    meilisearch::MeilisearchClient,
    routes::{
        db::{
            create_question, fetch_public_questions, fetch_question_topics, fetch_review_questions,
            find_admin_question, find_public_question, set_question_status, update_question,
        },
        delegates::delegate_questions::{
            mail::send_question_mail, recipients::find_delegate_contact,
        },
    },
};

const MAX_SUBJECT_LENGTH: usize = 255;
const MAX_BODY_LENGTH: usize = 10_000;

pub async fn ask_delegate_question_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Query(query): Query<DelegateQuestionQuery>,
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
        &question.eurovoc_topic_ids,
    )
    .await?;

    let topics = fetch_question_topics(&pg, &[question_id], query.language).await?;

    Ok(Json(DelegateQuestionCreated {
        id: question_id,
        delivery: delegate.delivery,
        recipient_name: delegate.recipient_name,
        status: "pending".to_string(),
        topics: topics.get(&question_id).cloned().unwrap_or_default(),
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
    Query(query): Query<DelegateQuestionQuery>,
    Path(delegate_id): Path<i32>,
) -> Result<Json<Vec<PublicDelegateQuestion>>, GenericError> {
    fetch_public_questions(&pg, Some(delegate_id), query.language)
        .await
        .map(Json)
}

pub async fn all_delegate_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<DelegateQuestionQuery>,
) -> Result<Json<Vec<PublicDelegateQuestion>>, GenericError> {
    fetch_public_questions(&pg, None, query.language)
        .await
        .map(Json)
}

pub async fn pending_delegate_questions_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Query(query): Query<DelegateQuestionQuery>,
) -> Result<Json<Vec<AdminDelegateQuestion>>, GenericError> {
    ensure_admin(&claims)?;
    fetch_review_questions(&pg, query.language).await.map(Json)
}

pub async fn approve_delegate_question_route(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    PgPoolConnection(pg): PgPoolConnection,
    ParliamentCtx(parliament): ParliamentCtx,
    claims: Claims,
    Query(query): Query<DelegateQuestionQuery>,
    Path(question_id): Path<i64>,
) -> Result<Json<AdminDelegateQuestion>, GenericError> {
    ensure_admin(&claims)?;

    let question = find_admin_question(&pg, question_id, query.language).await?;
    if question.status != "pending" && question.status != "failed" {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be approved",
        )));
    }

    send_question_mail(&pg, question_id, &meilisearch_client, parliament).await?;
    find_admin_question(&pg, question_id, query.language)
        .await
        .map(Json)
}

pub async fn reject_delegate_question_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Query(query): Query<DelegateQuestionQuery>,
    Path(question_id): Path<i64>,
) -> Result<Json<AdminDelegateQuestion>, GenericError> {
    ensure_admin(&claims)?;
    let question = find_admin_question(&pg, question_id, query.language).await?;
    if question.status != "pending" && question.status != "failed" {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be rejected",
        )));
    }

    set_question_status(&pg, question_id, "rejected").await?;
    find_admin_question(&pg, question_id, query.language)
        .await
        .map(Json)
}

pub async fn delegate_question_by_id_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<DelegateQuestionQuery>,
    Path(question_id): Path<i64>,
) -> Result<Json<PublicDelegateQuestion>, GenericError> {
    find_public_question(&pg, question_id, query.language)
        .await
        .map(Json)
}

pub async fn update_delegate_question_route(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    PgPoolConnection(pg): PgPoolConnection,
    ParliamentCtx(parliament): ParliamentCtx,
    claims: Claims,
    Query(query): Query<DelegateQuestionQuery>,
    Path(question_id): Path<i64>,
    Json(update): Json<UpdateDelegateQuestion>,
) -> Result<Json<AdminDelegateQuestion>, GenericError> {
    ensure_admin(&claims)?;

    let question = find_admin_question(&pg, question_id, query.language).await?;
    if question.status != "pending" && question.status != "failed" {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be updated",
        )));
    }

    let subject = update.subject.map(|subject| subject.trim().to_owned());
    let body = update.body.map(|body| body.trim().to_owned());
    let topic_ids = update.eurovoc_topic_ids.as_deref();

    if subject.is_none() && body.is_none() && topic_ids.is_none() {
        return Err(GenericError::Custom((
            StatusCode::BAD_REQUEST,
            "Nothing to update",
        )));
    }

    validate_question(
        subject.as_deref().unwrap_or(&question.subject),
        body.as_deref().unwrap_or(&question.body),
    )?;

    update_question(
        &pg,
        question_id,
        subject.as_deref(),
        body.as_deref(),
        topic_ids,
    )
    .await?;

    let language = match parliament {
        Parliament::At => Language::De,
        Parliament::Eu => Language::En,
    };
    let question = find_public_question(&pg, question_id, language).await?;
    meilisearch_client
        .index(Index::DelegateQuestions.as_str())
        .add_documents(&[question], None)
        .await
        .map_err(|e| GenericError::MeilisearchFailure(e))?;

    find_admin_question(&pg, question_id, query.language)
        .await
        .map(Json)
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
