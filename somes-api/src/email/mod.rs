// mod read_mailbox;

use dotenvy_macro::dotenv;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;

use crate::EMAIL_EXPIRATION_SECONDS;

// env vars?

pub const SMTP_USERNAME: &str = dotenv!("SMTP_USERNAME");
pub const SMTP_PASSWORD: &str = dotenv!("SMTP_PASSWORD");
pub const MAIL_FROM_DISPLAY: &str = dotenv!("MAIL_FROM_DISPLAY");
pub const MAIL_SERVER: &str = dotenv!("MAIL_SERVER");
pub const SMTP_PORT: &str = dotenv!("SMTP_PORT");
pub const SMTP_TLS: &str = dotenv!("SMTP_TLS");

pub const EMAIL_TEMPLATE: &str = include_str!("email_template.html");

pub static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    log::info!("SMTP USER: {}", SMTP_USERNAME);
    let port = SMTP_PORT
        .parse::<u16>()
        .expect("SMTP_PORT must be a valid port");
    let use_tls = SMTP_TLS != "false";
    log::info!(
        "Connecting to email relay at {}:{} (tls={})...",
        MAIL_SERVER,
        port,
        use_tls
    );

    let mut builder = if use_tls {
        SmtpTransport::starttls_relay(MAIL_SERVER).expect("Email relay not available.")
    } else {
        SmtpTransport::builder_dangerous(MAIL_SERVER)
    };

    builder = builder.port(port);

    if !SMTP_USERNAME.is_empty() || !SMTP_PASSWORD.is_empty() {
        builder = builder.credentials(Credentials::new(
            SMTP_USERNAME.to_string(),
            SMTP_PASSWORD.to_string(),
        ));
    }

    builder.build()
});

pub fn send_mail(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    send_mail_with_message_id(mailer, mail_to, subject, content, None)
}

pub fn send_mail_with_message_id(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
    message_id: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = MAIL_FROM_DISPLAY.parse()?;
    let to = format!("Recipient <{mail_to}>").parse()?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .message_id(message_id)
        .body(content)?;

    let response = mailer.send(&email)?;
    log::info!("Sent email to {mail_to}: {response:?}");
    Ok(())
}

pub fn send_otp_mail(mail_to: &str, otp: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut white_splitted = String::new();
    for (idx, ch) in otp.chars().enumerate() {
        white_splitted.push(ch);
        if (idx + 1) % 3 == 0 {
            white_splitted.push(' ')
        }
    }
    let content = EMAIL_TEMPLATE.replace("{*OTP*}", &white_splitted);
    let content = content.replace("{*MINUTOS*}", &(*EMAIL_EXPIRATION_SECONDS / 60).to_string());

    send_mail(&MAILER, mail_to, "Dein Somes One-Time Passwort", content)?;

    Ok(())
}
