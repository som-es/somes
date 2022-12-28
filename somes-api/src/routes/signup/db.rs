use crate::db::model::{NewUser, User};
use crate::db::schema::user::dsl::*;
use crate::hash;
use diesel::prelude::*;
use somes_common_lib::SignUpInfo;

use super::error::SignUpErrorResponse;

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
    use somes_common_lib::SignUpInfo;
    use crate::{db::schema::user::dsl::user, routes::signup::db::{is_email_in_database, is_username_in_database}, model::User};
    use diesel::RunQueryDsl;
    use crate::{test_db, id, routes::signup::db::insert_new_user};


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

}
