// mod read_mailbox;

use dotenvy_macro::dotenv;
use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, client::TlsParameters},
    Message, SmtpTransport, Transport,
};
use once_cell::sync::Lazy;

use crate::EMAIL_EXPIRATION_SECONDS;

// env vars?

pub const SMTP_USERNAME: &str = dotenv!("SMTP_USERNAME");
pub const SMTP_PASSWORD: &str = dotenv!("SMTP_PASSWORD");
pub const MAIL_FROM_DISPLAY: &str = dotenv!("MAIL_FROM_DISPLAY");
pub const MAIL_SERVER: &str = dotenv!("MAIL_SERVER");

pub const EMAIL_TEMPLATE: &str = include_str!("email_template.html");

pub static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());
    log::info!("Connecting to email relay...");
    let tls_parameters = TlsParameters::builder(MAIL_SERVER.to_string())
        .dangerous_accept_invalid_certs(true)
        .build()
        .expect("Failed to build TLS parameters");

    SmtpTransport::relay(MAIL_SERVER)
        .expect("Email relay not available.")
        .credentials(creds)
        .tls(lettre::transport::smtp::client::Tls::Wrapper(
            tls_parameters,
        ))
        .build()
});

pub fn send_mail(
    mailer: &SmtpTransport,
    mail_to: &str,
    subject: &str,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = format!("somes auth <{}>", SMTP_USERNAME).parse()?;
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

#[test]
fn test_send_mail() {
    send_otp_mail("robin.pautsch@it.htl-hl.ac.at", "A12 3Z2 HAC").unwrap()
    // send_mail("", "tolle_id_zum_verifizieren");
}
