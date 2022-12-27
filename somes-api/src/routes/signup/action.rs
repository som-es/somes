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
    signup_info: &SignUpInfo,
) -> Result<(), SignUpErrorResponse> {
    let mut sign_up_error = SignUpErrorWrapper::default();

    if signup_info.email.is_empty() {
        set_error_true!(sign_up_error, missing_email);
    }

    if signup_info.password.is_empty() {
        set_error_true!(sign_up_error, missing_password);
    }

    if signup_info.username.is_empty() {
        set_error_true!(sign_up_error, missing_username);
    }

    if !EMAIL_REGEX.is_match(&signup_info.email) {
        set_error_true!(sign_up_error, invalid_email);
    }

    if measure_password_strength(&signup_info.password) == Strength::Weak {
        set_error_true!(sign_up_error, insufficient_password);
    }

    if is_email_in_database(con, &signup_info.email) {
        set_error_true!(sign_up_error, email_taken);
    }

    if is_username_in_database(con, &signup_info.username) {
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
    use crate::db::schema::user::dsl::user;
    use diesel::RunQueryDsl;
    use somes_common_lib::SignUpInfo;

    use crate::model::User;
    use crate::{
        id,
        routes::signup::{
            action::{insert_new_user, is_username_in_database},
            error::SignUpErrorResponse,
        },
        test_db,
    };

    use super::{is_email_in_database, validate_signup_info};

    #[test]
    fn test_insert_new_user() {
        let h = test_db::create_handle(id!());
        let con = &mut h.establish_connection();

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        insert_new_user(con, &signup_info).unwrap();

        let first_user = &user.load::<User>(con).unwrap()[0];

        assert_eq!(first_user.email, signup_info.email);
        assert_eq!(first_user.username, signup_info.username);
    }

    #[test]
    fn test_email_in_database() {
        let h = test_db::create_handle(id!());
        let con = &mut h.establish_connection();

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        insert_new_user(con, &signup_info).unwrap();

        assert!(!is_email_in_database(con, "email"));
        assert!(is_email_in_database(con, "test@test.at"));
    }

    #[test]
    fn test_username_in_database() {
        let h = test_db::create_handle(id!());
        let con = &mut h.establish_connection();

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        insert_new_user(con, &signup_info).unwrap();

        assert!(!is_username_in_database(con, "stefan"));
        assert!(is_username_in_database(con, "test_name"));
    }

    #[test]
    fn test_validate_signup_info_short_password() {
        let h = test_db::create_handle(id!());

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        let con = &mut h.establish_connection();
        match validate_signup_info(con, &signup_info) {
            Ok(_) => panic!("Not possible, password is weak"),
            Err(err) => match err {
                SignUpErrorResponse::UserCreationError => panic!(""),
                SignUpErrorResponse::SignUpError(signup_err) => {
                    assert!(signup_err.insufficient_password);
                    assert!(!signup_err.invalid_email);
                    assert!(!signup_err.email_taken);
                }
            },
        }
    }

    #[test]
    fn test_validate_signup_info_taken() {
        let h = test_db::create_handle(id!());

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        let con = &mut h.establish_connection();

        insert_new_user(con, &signup_info).unwrap();

        match validate_signup_info(con, &signup_info) {
            Ok(_) => panic!("Not possible, password is weak"),
            Err(err) => match err {
                SignUpErrorResponse::UserCreationError => panic!(""),
                SignUpErrorResponse::SignUpError(signup_err) => {
                    assert!(signup_err.insufficient_password);
                    assert!(signup_err.email_taken);
                    assert!(signup_err.username_taken);
                }
            },
        }
    }

    #[test]
    fn test_validate_signup_info_missing_entries() {
        let h = test_db::create_handle(id!());

        let signup_info = SignUpInfo {
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
        };

        let con = &mut h.establish_connection();
        match validate_signup_info(con, &signup_info) {
            Ok(_) => panic!("Not possible, password is weak"),
            Err(err) => match err {
                SignUpErrorResponse::UserCreationError => panic!(""),
                SignUpErrorResponse::SignUpError(signup_err) => {
                    assert!(signup_err.insufficient_password);
                    assert!(!signup_err.email_taken);
                    assert!(signup_err.missing_username);
                    assert!(signup_err.missing_email);
                    assert!(signup_err.missing_password);
                }
            },
        }
    }
}
