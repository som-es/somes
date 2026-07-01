# Delegate Question Mail Listener

This service polls an inbound mailbox, matches replies through `In-Reply-To` and `References`, and stores matching answers in the dataservice database.

The service shares mail parsing and message ID handling with the API through the `delegate-question-mail` library crate.

## Local Mailpit

Mailpit is only used as a local test mailbox. Start it with POP3 enabled and configure the listener as follows:

```env
MAIL_INBOUND_PROTOCOL=pop3
MAIL_INBOUND_HOST=127.0.0.1
MAIL_INBOUND_PORT=1110
MAIL_INBOUND_USERNAME=test
MAIL_INBOUND_PASSWORD=test
MAIL_INBOUND_POLL_SECONDS=5
```

The existing `DATASERVICE_URL` variable is also required. Start the listener from the workspace root:

```bash
cargo run -p delegate-question-mail-listener
```

## Production IMAP

Use a dedicated inbound mailbox for question replies:

```env
MAIL_INBOUND_PROTOCOL=imap
MAIL_INBOUND_HOST=imap.example.org
MAIL_INBOUND_PORT=993
MAIL_INBOUND_USERNAME=questions@example.org
MAIL_INBOUND_PASSWORD=replace-me
MAIL_INBOUND_MAILBOX=INBOX
MAIL_INBOUND_POLL_SECONDS=15
```

The listener does not send mail. It only reads incoming messages and records replies whose headers reference a stored outgoing question message ID.
