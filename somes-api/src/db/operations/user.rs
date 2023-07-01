use crate::db::model::{NewUser, User};
use crate::db::schema::users::dsl::*;
use crate::hash;
use diesel::prelude::*;
use diesel::PgConnection;
use somes_common_lib::SignUpInfo;

pub fn is_email_in_db(con: &mut PgConnection, signup_email: &str) -> bool {
    users
        .filter(email.eq(signup_email))
        // .first::<User>(con)
        .first::<User>(con)
        .is_ok()
}

pub fn is_username_in_db(con: &mut PgConnection, signup_username: &str) -> bool {
    users
        .filter(username.eq(signup_username))
        .first::<User>(con)
        .is_ok()
}

pub fn update_password_hash_at(
    con: &mut PgConnection,
    user: &str,
    new_password_hash: &str,
) -> QueryResult<usize> {
    diesel::update(users.filter(username.eq(user)))
        .set(password_hash.eq(new_password_hash))
        .execute(con)
}


/// MIND: this is a general purpose user getting function. It also returns the password hash!
/// Do not forget to remove the password hash from the user before returning it to the client!
/// As `User´ does not implement `Serialize´, this should not happen in the first case. Be careful nonetheless!
pub fn get_user_from_db_by_id(con: &mut PgConnection, val_id: i32) -> Option<User> {
    users.find(val_id).first::<User>(con).ok()
}

pub fn get_user_from_db(
    con: &mut PgConnection,
    login_username: &str,
    login_email: &str,
) -> Option<User> {
    users
        .filter(username.eq(login_username))
        .or_filter(email.eq(login_email))
        .first::<User>(con)
        .ok()
}

pub fn get_password_hash_from_db(
    con: &mut PgConnection,
    login_username: &str,
    login_email: &str,
) -> Option<String> {
    users
        .filter(username.eq(login_username))
        .or_filter(email.eq(login_email))
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

impl TryFrom<SignUpInfo> for NewUser {
    type Error = argon2::password_hash::Error;

    fn try_from(signup_info: SignUpInfo) -> argon2::password_hash::Result<Self> {
        NewUser::new(
            signup_info.email,
            signup_info.username,
            &signup_info.password,
        )
    }
}

pub fn insert_user(con: &mut PgConnection, new_user: &NewUser) -> QueryResult<i32> {
    diesel::insert_into(users)
        .values(new_user)
        .returning(id)
        .get_result(con)
}

#[cfg(test)]
mod tests {
    use crate::{
        db::schema::users::dsl::users,
        establish_connection,
        model::{NewUser, User},
        operations::user::{insert_user, is_email_in_db, is_username_in_db},
    };
    use diesel::{Connection, RunQueryDsl};
    use somes_common_lib::SignUpInfo;

    use super::{get_password_hash_from_db, get_user_from_db, update_password_hash_at};

    #[test]
    fn test_insert_new_user() {
        let con = &mut establish_connection();

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

        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

            let first_user = &users.load::<User>(con).unwrap()[0];

            assert_eq!(first_user.email, signup_info.email);
            assert_eq!(first_user.username, signup_info.username);

            Ok(())
        });
    }

    #[test]
    fn test_email_in_database() {
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

        let con = &mut establish_connection();

        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

            assert!(!is_email_in_db(con, "email"));
            assert!(is_email_in_db(con, "test@test.at"));
            Ok(())
        });
    }

    #[test]
    fn test_username_in_database() {
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

        let con = &mut establish_connection();

        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

            assert!(!is_username_in_db(con, "stefan"));
            assert!(is_username_in_db(con, "test_name"));
            Ok(())
        });
    }

    #[test]
    fn test_get_user_from_db() {
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

        let con = &mut establish_connection();

        con.test_transaction::<_, (), _>(|con| {
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

            assert!(db_user.is_none());
            Ok(())
        });
    }

    #[test]
    fn test_get_password_hash_from_db() {
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

        let con = &mut establish_connection();

        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

            let pw_hash = get_password_hash_from_db(con, "test_name", "").unwrap();
            assert!(pw_hash.len() > signup_info.password.len());
            Ok(())
        });
    }

    #[test]
    fn test_update_password_hash() {
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

        let con = &mut establish_connection();

        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

            update_password_hash_at(con, "test_name", "new_pw_hash");
            let pw_hash = get_password_hash_from_db(con, "test_name", "").unwrap();
            assert_eq!(pw_hash, "new_pw_hash");
            Ok(())
        });
    }
}
