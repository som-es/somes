// mod read_mailbox;

use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;

use crate::EMAIL_EXPIRATION_SECONDS;

// env vars?

pub static SMTP_USERNAME: Lazy<String> = Lazy::new(|| crate::env_var("SMTP_USERNAME"));
pub static SMTP_PASSWORD: Lazy<String> = Lazy::new(|| crate::env_var("SMTP_PASSWORD"));
pub static MAIL_FROM_DISPLAY: Lazy<String> = Lazy::new(|| crate::env_var("MAIL_FROM_DISPLAY"));
pub static MAIL_SERVER: Lazy<String> = Lazy::new(|| crate::env_var("MAIL_SERVER"));

pub const EMAIL_TEMPLATE: &str = include_str!("email_template.html");

pub static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());
    log::info!("SMTP USER: {}", SMTP_USERNAME.as_str());
    log::info!("Connecting to email relay...");

    // let tls_parameters = TlsParameters::builder(MAIL_SERVER.to_string())
    //     // .dangerous_accept_invalid_certs(true)
    //     .build()
    //     .expect("Failed to build TLS parameters");

    SmtpTransport::starttls_relay(&*MAIL_SERVER)
        .expect("Email relay not available.")
        .credentials(creds)
        // .tls(lettre::transport::smtp::client::Tls::Wrapper(
        //     tls_parameters,
        // ))
        .build()
});

pub fn send_mail(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = format!("somes auth <{}>", SMTP_USERNAME.as_str()).parse()?;
    let to = format!("Recipient <{mail_to}>").parse()?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(content)?;

    mailer.send(&email)?;
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
