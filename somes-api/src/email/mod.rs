// mod read_mailbox;

use dotenvy_macro::dotenv;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use somes_common_lib::SEND_MAIL_INFO;

use crate::EMAIL_EXPIRATION_SECONDS;

// env vars?

pub const SMTP_USERNAME: &str = dotenv!("SMTP_USERNAME");
pub const SMTP_PASSWORD: &str = dotenv!("SMTP_PASSWORD");
pub const MAIL_FROM_DISPLAY: &str = dotenv!("MAIL_FROM_DISPLAY");
pub const MAIL_SERVER: &str = dotenv!("MAIL_SERVER");
pub const SMTP_PORT: &str = dotenv!("SMTP_PORT");
pub const SMTP_TLS: &str = dotenv!("SMTP_TLS");

pub const QUESTION_SMTP_USERNAME: &str = dotenv!("QUESTION_SMTP_USERNAME");
pub const QUESTION_SMTP_PASSWORD: &str = dotenv!("QUESTION_SMTP_PASSWORD");
pub const QUESTION_MAIL_FROM_DISPLAY: &str = dotenv!("QUESTION_MAIL_FROM_DISPLAY");
pub const QUESTION_MAIL_SERVER: &str = dotenv!("QUESTION_MAIL_SERVER");
pub const QUESTION_SMTP_PORT: &str = dotenv!("QUESTION_SMTP_PORT");
pub const QUESTION_SMTP_TLS: &str = dotenv!("QUESTION_SMTP_TLS");

pub const EMAIL_TEMPLATE: &str = include_str!("email_template.html");

pub fn create_mailer(
    host: &str,
    port: u16,
    use_tls: bool,
    smtp_username: &str,
    smtp_passowrd: &str,
) -> SmtpTransport {
    log::info!("SMTP USER: {}", smtp_username);

    log::info!(
        "Connecting to email relay at {}:{} (tls={})...",
        host,
        port,
        use_tls
    );

    let mut builder = if use_tls {
        SmtpTransport::starttls_relay(host).expect("Email relay not available.")
    } else {
        SmtpTransport::builder_dangerous(host)
    };

    builder = builder.port(port);

    if !smtp_username.is_empty() || !smtp_passowrd.is_empty() {
        builder = builder.credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_passowrd.to_string(),
        ));
    }

    builder.build()
}

pub static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let port = SMTP_PORT
        .parse::<u16>()
        .expect("SMTP_PORT must be a valid port");
    let use_tls = SMTP_TLS != "false";
    create_mailer(MAIL_SERVER, port, use_tls, SMTP_USERNAME, SMTP_PASSWORD)
});

pub static QUESTION_MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let port = QUESTION_SMTP_PORT
        .parse::<u16>()
        .expect("SMTP_PORT must be a valid port");
    let use_tls = QUESTION_SMTP_TLS != "false";
    create_mailer(
        QUESTION_MAIL_SERVER,
        port,
        use_tls,
        QUESTION_SMTP_USERNAME,
        QUESTION_SMTP_PASSWORD,
    )
});

pub fn send_mail(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
    from: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    send_mail_with_message_id(mailer, mail_to, subject, content, None, from)
}

pub fn send_mail_with_message_id(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
    message_id: Option<String>,
    from: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = from.parse()?;
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

    send_mail(
        &MAILER,
        mail_to,
        "Dein Somes One-Time Passwort",
        content,
        SEND_MAIL_INFO,
    )?;

    Ok(())
}
