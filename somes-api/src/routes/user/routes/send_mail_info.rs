use axum::Json;
use somes_common_lib::SendMailInfo;
use sqlx::{query, query_as};

use crate::{GenericError, PgPoolConnection, jwt::Claims};

pub async fn update_send_mail_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Json(mail_info): Json<SendMailInfo>,
) -> Result<Json<()>, GenericError> {
    query!(
        "insert into user_notification_settings (
            user_id,
            platform,
            send_new_vote_results,
            send_new_vote_result_by_favo,
            send_new_delegate_activity,
            send_new_ministrial_prop,
            send_new_ministrial_prop_by_favo,
            send_new_decree,
            send_new_decree_by_favo,
            send_new_proposal,
            send_new_proposal_by_favo
        ) values ($10, 'web', $1, $9, $2, $3, $4, $5, $6, $7, $8)
        on conflict (user_id, platform) do update set
            send_new_vote_results = EXCLUDED.send_new_vote_results,
            send_new_vote_result_by_favo = EXCLUDED.send_new_vote_result_by_favo,
            send_new_delegate_activity = EXCLUDED.send_new_delegate_activity,
            send_new_ministrial_prop = EXCLUDED.send_new_ministrial_prop,
            send_new_ministrial_prop_by_favo = EXCLUDED.send_new_ministrial_prop_by_favo,
            send_new_decree = EXCLUDED.send_new_decree,
            send_new_decree_by_favo = EXCLUDED.send_new_decree_by_favo,
            send_new_proposal = EXCLUDED.send_new_proposal,
            send_new_proposal_by_favo = EXCLUDED.send_new_proposal_by_favo,
            updated_at = now()",
        mail_info.send_new_vote_results_mails,
        mail_info.send_new_delegate_activity_mails,
        mail_info.send_new_ministrial_prop_mails,
        mail_info.send_new_ministrial_prop_by_favo_mails,
        mail_info.send_new_decree_mails,
        mail_info.send_new_decree_by_favo_mails,
        mail_info.send_new_proposal_mails,
        mail_info.send_new_proposal_by_favo_mails,
        mail_info.send_new_vote_result_by_favo_mails,
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
        "select
            coalesce(s.send_new_vote_results, true)::bool as \"send_new_vote_results_mails!\",
            coalesce(s.send_new_delegate_activity, true)::bool as \"send_new_delegate_activity_mails!\",
            coalesce(s.send_new_ministrial_prop, false)::bool as \"send_new_ministrial_prop_mails!\",
            coalesce(s.send_new_ministrial_prop_by_favo, false)::bool as \"send_new_ministrial_prop_by_favo_mails!\",
            coalesce(s.send_new_decree, false)::bool as \"send_new_decree_mails!\",
            coalesce(s.send_new_decree_by_favo, false)::bool as \"send_new_decree_by_favo_mails!\",
            coalesce(s.send_new_proposal, false)::bool as \"send_new_proposal_mails!\",
            coalesce(s.send_new_proposal_by_favo, false)::bool as \"send_new_proposal_by_favo_mails!\",
            coalesce(s.send_new_vote_result_by_favo, false)::bool as \"send_new_vote_result_by_favo_mails!\"
        from (select 1::int as id) u
        left join user_notification_settings s on s.user_id = $1 and s.platform = 'web'",
        claims.id,
    )
    .fetch_one(&pg)
    .await
    .map(Json)
    .map_err(|_| GenericError::SqlFailure(None));
    mail_info
}
