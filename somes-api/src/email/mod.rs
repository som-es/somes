use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use once_cell::sync::Lazy;

use crate::{CONTENT, SUBJECT};

// env vars?

static SMTP_USERNAME: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("./src/email/smtp_username_secret")
        .expect("Can't open smtp username secret file!")
});

static SMTP_PASSWORD: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("./src/email/smtp_password_secret")
        .expect("Can't open smtp password secret file!")
});

static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());
    SmtpTransport::relay("zimbra.nagy-blumen.at")
        .expect("Email relay not available.")
        .credentials(creds)
        .build()
});

pub fn send_mail(mail_to: &str, verification_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let from = format!("somes <{}>", *SMTP_USERNAME).parse()?;
    let to = format!("Recipient <{mail_to}>").parse()?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(SUBJECT)
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "{CONTENT}
http://somes.at/verify?id={verification_id}
        "
        ))?;

    MAILER.send(&email)?;
    Ok(())
}

#[test]
fn test_send_mail() {
    // send_mail("", "tolle_id_zum_verifizieren");
}
