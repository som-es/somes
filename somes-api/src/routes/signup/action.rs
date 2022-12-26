use diesel::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use somes_common_lib::SignUpInfo;
use crate::db::model::User;
use crate::db::schema::user::dsl::*;

use crate::MIN_PASSWORD_LEN;

use super::error::SignUpError;

static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*@[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*$").unwrap());

/// checks the validity of the signup info
pub fn validate_signup_info(con: &mut SqliteConnection, sign_up_info: &SignUpInfo) -> Result<(), SignUpError> {
    if sign_up_info.email.is_empty() {
        return Err(SignUpError::MissingEmail);
    }

    if sign_up_info.password.is_empty() {
        return Err(SignUpError::MissingPassword);
    }

    if sign_up_info.username.is_empty() {
        return Err(SignUpError::MissingUsername);
    }

    if !EMAIL_REGEX.is_match(&sign_up_info.email) {
        return Err(SignUpError::InvalidEmail);
    }
    
    if sign_up_info.password.len() < *MIN_PASSWORD_LEN {
        return Err(SignUpError::InsufficientPassword);
    }

    if is_email_in_database(con, &sign_up_info.email) {
        return Err(SignUpError::EMailTaken);
    }

    Ok(())
}

pub fn is_email_in_database(con: &mut SqliteConnection, signup_email: &str) -> bool {
    user.filter(email.is(signup_email)).first::<User>(con).is_ok()
}