use axum::Json;
use common_scrapes::Contact;
use common_scrapes::Delegate;
use dataservice::db::api_models::DbQuestion;
use dataservice::db::models::DbContact;
use dataservice::db::models::DbContactQuery;
use dataservice::db::models::DbDelegate;
use dataservice::db::schema::contacts::dsl::contacts;
use dataservice::db::schema::contacts::dsl::id as contacts_id;
use dataservice::db::schema::delegates::{dsl::delegates, id};
use dataservice::db::schema::questions::dsl::questions;
use dataservice::db::schema::questions::id as question_id;
use diesel::insert_into;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel::RunQueryDsl;
use somes_common_lib::AskQuestion;

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
    con
        .interact(move |con| {
            insert_question(con, issuer_id, insert_title, insert_body, delegate_id).map_err(|e| {
                match e {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => QuestionErrorResponse::DuplicateQuestion,
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                        _,
                    ) => QuestionErrorResponse::InvalidDelegate,
                    _ => QuestionErrorResponse::DbInteraction,
                }
            })
        })
        .await
        .map_err(|_| QuestionErrorResponse::DbInteraction)?
}

pub async fn has_delegate_account() -> bool {
    false
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
    let new_question_id = insert_new_question(&con, claims.id, title.clone(), body.clone(), delegate_id).await?;

    let title = format!("[{new_question_id}] {title}");
    todo!()
}

pub fn insert_question(
    con: &mut PgConnection,
    issuer_id: i32,
    title: String,
    body: String,
    delegate_id: i32,
) -> QueryResult<i32> {
    let db_question = DbQuestion {
        issuer_id,
        delegate_id,
        title,
        body,
    };

    insert_into(questions)
        .values(db_question)
        .returning(question_id)
        .get_result(con)
}
