use crate::db::model::{NewUser, User};
use crate::db::schema::user::dsl::*;
use crate::hash;
use diesel::prelude::*;

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

impl NewUser {
    pub fn new(
        signup_email: String,
        signup_username: String,
        password: &str,
    ) -> argon2::password_hash::Result<Self> {
        Ok(NewUser {
            username: signup_username,
            email: signup_email,
            password_hash: hash::hash_password(password)?,
        })
    }
}

pub fn insert_user(con: &mut SqliteConnection, new_user: &NewUser) -> QueryResult<()> {
    diesel::insert_into(user).values(new_user).execute(con)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        db::schema::user::dsl::user,
        model::{NewUser, User},
        operations::user::{insert_user, is_email_in_database, is_username_in_database},
    };
    use crate::{id, test_db};
    use diesel::RunQueryDsl;
    use somes_common_lib::SignUpInfo;

    #[test]
    fn test_insert_new_user() {
        let h = test_db::create_handle(id!());
        let con = &mut h.establish_connection();

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };

        let new_user = NewUser::new(
            signup_info.email.clone(),
            signup_info.username.clone(),
            &signup_info.password,
        )
        .unwrap();
        insert_user(con, &new_user).unwrap();

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
        let new_user = NewUser::new(
            signup_info.email,
            signup_info.username,
            &signup_info.password,
        )
        .unwrap();
        insert_user(con, &new_user).unwrap();

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
        let new_user = NewUser::new(
            signup_info.email,
            signup_info.username,
            &signup_info.password,
        )
        .unwrap();
        insert_user(con, &new_user).unwrap();

        assert!(!is_username_in_database(con, "stefan"));
        assert!(is_username_in_database(con, "test_name"));
    }
}
