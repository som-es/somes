use crate::db::model::{NewUser, User};
use crate::db::schema::user::dsl::*;
use crate::hash;
use diesel::prelude::*;

pub fn is_email_in_db(con: &mut SqliteConnection, signup_email: &str) -> bool {
    user.filter(email.is(signup_email))
        .first::<User>(con)
        .is_ok()
}

pub fn is_username_in_db(con: &mut SqliteConnection, signup_username: &str) -> bool {
    user.filter(username.is(signup_username))
        .first::<User>(con)
        .is_ok()
}

pub fn get_user_from_db(
    con: &mut SqliteConnection,
    login_username: &str,
    login_email: &str,
) -> Option<User> {
    user.filter(username.is(login_username))
        .or_filter(email.is(login_email))
        .first::<User>(con)
        .ok()
}

pub fn get_password_hash_from_db(
    con: &mut SqliteConnection,
    login_username: &str,
    login_email: &str,
) -> Option<String> {
    user.filter(username.is(login_username))
        .or_filter(email.is(login_email))
        .select(password_hash)
        .first::<String>(con)
        .ok()
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
        operations::user::{insert_user, is_email_in_db, is_username_in_db},
    };
    use crate::{id, test_db};
    use diesel::RunQueryDsl;
    use somes_common_lib::SignUpInfo;

    use super::{get_password_hash_from_db, get_user_from_db};

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

        assert!(!is_email_in_db(con, "email"));
        assert!(is_email_in_db(con, "test@test.at"));
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

        assert!(!is_username_in_db(con, "stefan"));
        assert!(is_username_in_db(con, "test_name"));
    }

    #[test]
    fn test_get_user_from_db() {
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

        let db_user = get_user_from_db(con, "test_name", "").unwrap();

        assert_eq!("test_name", db_user.username);
        assert_eq!("test@test.at", db_user.email);
        assert_ne!("supersicher", db_user.password_hash);

        let db_user = get_user_from_db(con, "", "test@test.at").unwrap();

        assert_eq!("test_name", db_user.username);
        assert_eq!("test@test.at", db_user.email);
        assert_ne!("supersicher", db_user.password_hash);

        let db_user = get_user_from_db(con, "test_name", "test@test.at").unwrap();

        assert_eq!("test_name", db_user.username);
        assert_eq!("test@test.at", db_user.email);
        assert_ne!("supersicher", db_user.password_hash);

        let db_user = get_user_from_db(con, "test_account", "invalid");

        assert!(db_user.is_none())
    }

    #[test]
    fn test_get_password_hash_from_db() {
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

        let pw_hash = get_password_hash_from_db(con, "test_name", "").unwrap();
        assert!(pw_hash.len() > signup_info.password.len());
    }
}
