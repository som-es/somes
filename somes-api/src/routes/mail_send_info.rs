use axum::Json;
use serde_json::json;
use somes_common_lib::SendMailInfo;
use sqlx::query_as;

use crate::{jwt::Claims, PgPoolConnection};


pub async fn update_send_mail_info(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(delegate_favo): Json<SendMailInfo>,
) -> Result<Json<()>, Json<serde_json::Value>> {
    log::info!("sendm ail info: {delegate_favo:?} {}", claims.id);
    query_as!(
        UniqueTopic,
        "update somes_user set send_new_vote_results_mails = $1, send_new_delegate_activity_mails = $2, send_new_ministrial_prop_mails=$3 where id = $4",
        delegate_favo.send_new_vote_results_mails,
        delegate_favo.send_new_delegate_activity_mails,
        delegate_favo.send_new_ministrial_prop_mails,
        claims.id,
    )
    .execute(&pg)
    .await
    .map(|_| Json(()))
    .map_err(|_| Json(json!({"error": "db error"})))
}

pub async fn get_send_mail_info(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
) -> Result<Json<SendMailInfo>, Json<serde_json::Value>> {
    let mail_info = query_as!(
        SendMailInfo,
        "select send_new_vote_results_mails, send_new_delegate_activity_mails, send_new_ministrial_prop_mails from somes_user where id = $1",
        claims.id,
    )
    .fetch_one(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "db error"})));

    log::info!("mail info: {mail_info:?} {}", claims.id);

    mail_info
}