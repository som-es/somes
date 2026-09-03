use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

use crate::GenericError;

use super::models::{DelegateContact, QuestionDelivery};

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

/// Resolve who should actually receive a question to `delegate_id`: the delegate's own
/// address, or the configured party fallback.
pub(super) async fn find_delegate_contact(
    pg: &sqlx::PgPool,
    delegate_id: i32,
) -> Result<DelegateContact, GenericError> {
    let row = sqlx::query!(
        "
        SELECT d.name, d.party, c.mail
        FROM delegates d
        JOIN contacts c ON c.id = d.id
        WHERE d.id = $1
        ",
        delegate_id
    )
    .fetch_optional(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?
    .ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Delegate was not found",
    )))?;

    if let Some(email) = row.mail.filter(|email| !email.trim().is_empty()) {
        return Ok(DelegateContact {
            recipient_name: row.name.clone(),
            name: row.name,
            recipient_email: email,
            delivery: QuestionDelivery::Delegate,
        });
    }

    let party = row.party.ok_or(GenericError::Custom((
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
        name: row.name,
        recipient_name: recipient.name.clone(),
        recipient_email: recipient.email.clone(),
        delivery: QuestionDelivery::Party,
    })
}
