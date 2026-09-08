use combx::Parliament;
use reqwest::StatusCode;

use crate::{
    GenericError,
    email::{QUESTION_MAIL_FROM_DISPLAY, QUESTION_MAILER, send_mail_with_message_id},
    routes::update_question_in_meilisearch,
};

use super::{
    db::set_question_status,
    models::{DelegateContact, QuestionDelivery},
};

const DELEGATE_QUESTION_TEMPLATE: &str =
    include_str!("../../../email/delegate_question_template.html");
const PARTY_QUESTION_TEMPLATE: &str = include_str!("../../../email/party_question_template.html");

pub(super) async fn send_question_mail(
    pg: &sqlx::PgPool,
    question_id: i64,
    meilisearch_client: &meilisearch_sdk::client::Client,
    parliament: Parliament,
) -> Result<(), GenericError> {
    let row = sqlx::query!(
        "
        SELECT
            d.name AS delegate_name,
            q.recipient_email,
            q.recipient_kind,
            q.recipient_name,
            q.subject,
            q.body,
            q.outgoing_message_id
        FROM delegate_questions q
        JOIN delegates d ON d.id = q.delegate_id
        WHERE q.id = $1
        ",
        question_id
    )
    .fetch_optional(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?
    .ok_or(GenericError::Custom((
        StatusCode::NOT_FOUND,
        "Question was not found",
    )))?;

    let locked = sqlx::query(
        "
        UPDATE delegate_questions
        SET status = 'sending', updated_at = NOW()
        WHERE id = $1 AND status IN ('pending', 'failed')
        ",
    )
    .bind(question_id)
    .execute(pg)
    .await
    .map_err(|error| GenericError::SqlFailure(Some(error)))?;

    if locked.rows_affected() == 0 {
        return Err(GenericError::Custom((
            StatusCode::CONFLICT,
            "Question can not be approved",
        )));
    }

    let recipient_kind: String = row.recipient_kind;
    let delivery = match recipient_kind.as_str() {
        "delegate" => QuestionDelivery::Delegate,
        "party" => QuestionDelivery::Party,
        _ => {
            return Err(GenericError::Custom((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid question recipient kind",
            )));
        }
    };

    let delegate = DelegateContact {
        name: row.delegate_name,
        recipient_name: row.recipient_name,
        recipient_email: row.recipient_email,
        delivery,
    };
    let subject: String = row.subject;
    let body: String = row.body;
    let outgoing_message_id: String = row.outgoing_message_id;

    let mail_subject = format!("Neue Frage über somes.at: {subject}");
    let mail_content = render_question_mail(&delegate, &subject, &body);
    let recipient_email = delegate.recipient_email.clone();

    let mail_result = match tokio::task::spawn_blocking(move || {
        send_mail_with_message_id(
            &QUESTION_MAILER,
            &recipient_email,
            &mail_subject,
            mail_content,
            Some(outgoing_message_id),
            QUESTION_MAIL_FROM_DISPLAY,
        )
        .map_err(|error| error.to_string())
    })
    .await
    {
        Ok(result) => result,
        Err(error) => {
            log::error!("delegate question mail task failed: {error}");
            Err(error.to_string())
        }
    };

    match mail_result {
        Ok(()) => {
            set_question_status(pg, question_id, "sent").await?;

            update_question_in_meilisearch(meilisearch_client, &pg, parliament, question_id)
                .await?;

            Ok(())
        }
        Err(error) => {
            log::error!("sending delegate question {question_id} failed: {error}");
            set_question_status(pg, question_id, "failed").await?;
            Err(GenericError::Custom((
                StatusCode::BAD_GATEWAY,
                "Could not send question email",
            )))
        }
    }
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
    use super::{DelegateContact, QuestionDelivery, render_question_mail};

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
