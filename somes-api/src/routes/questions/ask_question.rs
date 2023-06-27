use axum::Json;
use common_scrapes::Contact;
use common_scrapes::Delegate;
use dataservice::db::models::DbContact;
use dataservice::db::models::DbContactQuery;
use dataservice::db::models::DbDelegate;
use dataservice::db::schema::contacts::dsl::contacts;
use dataservice::db::schema::contacts::dsl::id as contacts_id;
use dataservice::db::schema::delegates::{dsl::delegates, id};
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel::RunQueryDsl;
use somes_common_lib::AskQuestion;

use crate::DataserviceDbConnection;
use crate::jwt::Claims;

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
) -> Result<(DbDelegate, DbContactQuery), QuestionErrorResponse> {
    get_delegate_contact(con, delegate_id)
        .map_err(|_| QuestionErrorResponse::FetchDelegateContact)
        .and_then(|entries| {
            entries
                .get(0)
                .cloned()
                .ok_or(QuestionErrorResponse::InvalidDelegate)
        })
}
pub async fn ask_question(
    DataserviceDbConnection(con): DataserviceDbConnection,
    claims: Claims,
    Json(ask_question): Json<AskQuestion>,
) -> Result<(), QuestionErrorResponse> {
    let AskQuestion {
        delegate_id,
        text,
        body,
    } = ask_question;
    // has delegate a somes account

    let delegate_mail = con
        .interact(move |con| get_mail_from_delegate_id(con, delegate_id))
        .await
        .map_err(|_| QuestionErrorResponse::DbInteraction)??;

    todo!()
}
