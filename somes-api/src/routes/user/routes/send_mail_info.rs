use axum::Json;
use somes_common_lib::SendMailInfo;
use sqlx::query_as;

use crate::{jwt::Claims, GenericError, PgPoolConnection};

pub async fn update_send_mail_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<SendMailInfo>,
) -> Result<Json<()>, GenericError> {
    query_as!(
        UniqueTopic,
        "update somes_user set send_new_vote_results_mails = $1, send_new_delegate_activity_mails = $2, send_new_ministrial_prop_mails=$3, send_new_ministrial_prop_by_favo_mails=$4 where id = $5",
        delegate_favo.send_new_vote_results_mails,
        delegate_favo.send_new_delegate_activity_mails,
        delegate_favo.send_new_ministrial_prop_mails,
        delegate_favo.send_new_ministrial_prop_by_favo_mails,
        claims.id,
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}

pub async fn get_send_mail_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<SendMailInfo>, GenericError> {
    let mail_info = query_as!(
        SendMailInfo,
        "select send_new_vote_results_mails, send_new_delegate_activity_mails, send_new_ministrial_prop_mails, send_new_ministrial_prop_by_favo_mails from somes_user where id = $1",
        claims.id,
    )
    .fetch_one(&pg)
    .await
    .map(Json)
    .map_err(|_| GenericError::SqlFailure(None));
    mail_info
}
