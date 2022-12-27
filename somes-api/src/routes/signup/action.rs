use crate::db::model::{NewUser, User};
use crate::db::schema::user::dsl::*;
use crate::hash;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use somes_common_lib::password::{measure_password_strength, Strength};
use somes_common_lib::{set_error_true, SignUpInfo};

use super::error::{SignUpErrorResponse, SignUpErrorWrapper};

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*@[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*$").unwrap()
});

/// checks the validity of the signup info
pub fn validate_signup_info(
    con: &mut SqliteConnection,
    sign_up_info: &SignUpInfo,
) -> Result<(), SignUpErrorResponse> {
    let mut sign_up_error = SignUpErrorWrapper::default();

    if sign_up_info.email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if sign_up_info.password.is_empty() {
        set_error_true!(sign_up_error, missing_password);
    }

    if sign_up_info.username.is_empty() {
        set_error_true!(sign_up_error, missing_username);
    }

    if !EMAIL_REGEX.is_match(&sign_up_info.email) {
        set_error_true!(sign_up_error, invalid_email);
    }

    if measure_password_strength(&sign_up_info.password) == Strength::Weak {
        set_error_true!(sign_up_error, insufficient_password);
    }

    if is_email_in_database(con, &sign_up_info.email) {
        set_error_true!(sign_up_error, email_taken);
    }

    if is_username_in_database(con, &sign_up_info.email) {
        set_error_true!(sign_up_error, username_taken);
    }

    if sign_up_error.is_erroneous {
        return Err(SignUpErrorResponse::SignUpError(sign_up_error));
    }

    Ok(())
}

pub fn is_email_in_database(con: &mut SqliteConnection, signup_email: &str) -> bool {
    user.filter(email.is(signup_email))
        .first::<User>(con)
        .is_ok()
}

pub fn is_username_in_database(con: &mut SqliteConnection, signup_username: &str) -> bool {
    user.filter(username.is(signup_username))
        .first::<User>(con)
        .is_ok()
}

impl<'a> NewUser<'a> {
    pub fn new(
        signup_email: &'a str,
        signup_username: &'a str,
        password: &str,
    ) -> argon2::password_hash::Result<Self> {
        Ok(NewUser {
            username: signup_username,
            email: signup_email,
            password_hash: hash::hash_password(password)?,
        })
    }
}

pub fn insert_new_user(
    con: &mut SqliteConnection,
    signup_info: &SignUpInfo,
) -> Result<(), SignUpErrorResponse> {
    let new_user = NewUser::new(
        &signup_info.email,
        &signup_info.username,
        &signup_info.password,
    )
    .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    diesel::insert_into(user)
        .values(&new_user)
        .execute(con)
        .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::establish_connection;

    use super::is_email_in_database;

    #[test]
    fn test_email_in_database() {
        let con = &mut establish_connection();

        assert!(!is_email_in_database(con, "email"));
    }
}
