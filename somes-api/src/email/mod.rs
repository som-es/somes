use once_cell::sync::Lazy;

// env vars?

static SMTP_USERNAME: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("./src/email/smtp_username_secret")
        .expect("Can't open smtp username secret file!")
});

static SMTP_PASSWORD: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("./src/email/smtp_password_secret")
        .expect("Can't open smtp password secret file!")
});
