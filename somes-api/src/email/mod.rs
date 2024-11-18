// mod read_mailbox;

use dotenvy_macro::dotenv;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use once_cell::sync::Lazy;

use crate::{VERIFICATION_CONTENT, VERIFICATION_SUBJECT};

// env vars?

pub const SMTP_USERNAME: &str = dotenv!("SMTP_USERNAME");
pub const SMTP_PASSWORD: &str = dotenv!("SMTP_PASSWORD");
pub const MAIL_FROM_DISPLAY: &str = dotenv!("MAIL_FROM_DISPLAY");
pub const MAIL_SERVER: &str = dotenv!("MAIL_SERVER");

pub const EMAIL_TEMPLATE: &str = include_str!("email_template.html");

pub static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());
    log::info!("Connecting to email relay...");
    SmtpTransport::relay(MAIL_SERVER)
        .expect("Email relay not available.")
        .credentials(creds)
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

pub fn send_otp_mail(
    mail_to: &str,
    otp: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let content = EMAIL_TEMPLATE.replace("{*OTP*}", otp);

    send_mail(
        &MAILER,
        mail_to,
        "Dein Somes One-Time Passwort",
        content,
    )?;

    
    Ok(())
}

#[test]
fn test_send_mail() {
    send_otp_mail("florian.nagy@it.htl-hl.ac.at", "A12 3Z2 HAC").unwrap()
    // send_mail("", "tolle_id_zum_verifizieren");
}
