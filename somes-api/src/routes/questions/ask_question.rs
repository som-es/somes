use axum::Json;
use dataservice::db::{
    api_models::DbQuestion,
    models::{DbContactQuery, DbDelegate},
    
};
use diesel::{delete, insert_into, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl};
use diesel::{QueryResult, RunQueryDsl};
use somes_common_lib::AskQuestion;

use crate::email::send_mail;
use crate::email::MAILER;
use crate::jwt::Claims;
use crate::DataserviceDbConnection;

use super::error::QuestionErrorResponse;

#[inline]
pub fn get_delegate_contact(
    con: &mut PgConnection,
    delegate_id: i32,
) -> QueryResult<Vec<(DbDelegate, DbContactQuery)>> {
    delegates
        .inner_join(contacts.on(contacts_id.eq(id)))
        .filter(id.eq(delegate_id))
        .load::<(DbDelegate, DbContactQuery)>(con)
}

pub fn get_mail_from_delegate_id(
    con: &mut PgConnection,
    delegate_id: i32,
) -> Result<Option<String>, QuestionErrorResponse> {
    get_delegate_contact(con, delegate_id)
        .map_err(|_| QuestionErrorResponse::FetchDelegateContact)
        .and_then(|entries| {
            entries
                .get(0)
                .cloned()
                .ok_or(QuestionErrorResponse::InvalidDelegate)
                .map(|entry| entry.1.mail)
        })
}

pub async fn get_delegate_mail(
    con: &deadpool_diesel::postgres::Object,
    delegate_id: i32,
) -> Result<String, QuestionErrorResponse> {
    con.interact(move |con| get_mail_from_delegate_id(con, delegate_id))
        .await
        .map_err(|_| QuestionErrorResponse::DbInteraction)??
        .ok_or(QuestionErrorResponse::NoMailForDelegate)
}

pub async fn insert_new_question(
    con: &deadpool_diesel::postgres::Object,
    issuer_id: i32,
    insert_title: String,
    insert_body: String,
    delegate_id: i32,
) -> Result<i32, QuestionErrorResponse> {
    con.interact(move |con| {
        insert_question(con, issuer_id, insert_title, insert_body, delegate_id).map_err(|e| match e
        {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => QuestionErrorResponse::DuplicateQuestion,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                _,
            ) => QuestionErrorResponse::InvalidDelegate,
            _ => QuestionErrorResponse::DbInteraction,
        })
    })
    .await
    .map_err(|_| QuestionErrorResponse::DbInteraction)?
}

pub async fn has_delegate_account() -> bool {
    false
}

pub async fn handle_send_mail_and_remove_question_on_failure(
    delegate_mail: &str,
    title: &str,
    body: String,
    new_question_id: i32,
    con: &deadpool_diesel::postgres::Object,
) {
    let mut remove_question = false;
    if let Err(e) = send_mail(&MAILER, delegate_mail, title, body) {
        log::warn!("Error mailing question to delegate: {e}");
        remove_question = true;
    }

    if remove_question {
        log::info!("Traying to remove question from database...");
        let possibly_failed_interaction = con
            .interact(move |con| remove_question_by_id(con, new_question_id))
            .await;

        if possibly_failed_interaction
            .iter()
            .flatten()
            .next()
            .is_none()
        {
            log::error!("Could not remove question from database..")
        };
    }
}

pub async fn ask_question(
    DataserviceDbConnection(con): DataserviceDbConnection,
    claims: Claims,
    Json(ask_question): Json<AskQuestion>,
) -> Result<(), QuestionErrorResponse> {
    let AskQuestion {
        delegate_id,
        title,
        body,
    } = ask_question;

    // has delegate a somes account

    if has_delegate_account().await {
        // send message to account directly (without mail)
        return Ok(());
    }

    let delegate_mail = get_delegate_mail(&con, delegate_id).await?;
    let new_question_id =
        insert_new_question(&con, claims.id, title.clone(), body.clone(), delegate_id).await?;

    let title = format!("[{new_question_id}] {title}");

    // may not run?
    tokio::task::spawn(async move {
        handle_send_mail_and_remove_question_on_failure(
            &delegate_mail,
            &title,
            body,
            new_question_id,
            &con,
        )
        .await;
    });
    todo!()
}

use sqlx::PgPool;

pub async fn insert_question(
    con: &PgPool,
    issuer_id: i32,
    title: String,
    body: String,
    delegate_id: i32,
) -> sqlx::Result<i32> {
    let query = r#"
        INSERT INTO questions (issuer_id, delegate_id, title, body)
        VALUES ($1, $2, $3, $4)
        RETURNING question_id
    "#;

    let question_id: i32 = sqlx::query_scalar(query)
        .bind(issuer_id)
        .bind(delegate_id)
        .bind(title)
        .bind(body)
        .fetch_one(con)
        .await?;

    Ok(question_id)
}

pub async fn remove_question_by_id(con: &PgPool, val_question_id: i32) -> sqlx::Result<usize> {
    let query = r#"
        DELETE FROM questions
        WHERE question_id = $1
    "#;

    let rows_affected = sqlx::query(query)
        .bind(val_question_id)
        .execute(con)
        .await?
        .rows_affected();

    Ok(rows_affected as usize)
}
