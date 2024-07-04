use std::{env, error::Error};

use dataservice::db::schema::contacts::mail;
use imap::types::{Fetch, Fetches};

use super::{SMTP_PASSWORD, SMTP_USERNAME};

#[test]
fn test_read_mailbox() -> Result<(), Box<dyn Error>> {
    // Read config from environment or .env file
    let host = "zimbra.nagy-blumen.at".into();
    let user = SMTP_USERNAME.to_string();
    let password = SMTP_PASSWORD.to_string();
    let port = 993;

    let messages = fetch_inbox_top(host, user, password, port)?;
    if let Some(message) = messages.iter().next() {
        let env = message.envelope().unwrap();
        println!("env: {env:?}")
    }
    Ok(())
}

fn fetch_inbox_top(
    host: String,
    user: String,
    password: String,
    port: u16,
) -> Result<Fetches, Box<dyn Error>> {
    let client = imap::ClientBuilder::new(&host, port)
        .tls_kind(imap::TlsKind::Native)
        .connect()?;
    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client.login(&user, &password).map_err(|e| e.0)?;

    println!("Logged in");

    // we want to fetch the first email in the INBOX mailbox
    let mailbox = imap_session.select("INBOX")?;
    let next_uid = mailbox.uid_next.unwrap() - 1;
    println!("next_uid: {next_uid}");

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    // let messages = imap_session.fetch("1:*", "RFC822")?;

    let messages = imap_session.uid_fetch(next_uid.to_string(), "RFC822")?;
    imap_session.logout()?;
    Ok(messages)
    /*  let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };
    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;
    Ok(Some(body))*/
}
