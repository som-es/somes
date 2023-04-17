use diesel::prelude::*;
use once_cell::sync::Lazy;
use redis::AsyncCommands;
use regex::Regex;
use somes_common_lib::password::{measure_password_strength, Strength};

use somes_common_lib::{set_error_true, SignUpInfo};

use crate::model::NewUser;
use crate::operations::user::{is_email_in_db, is_username_in_db};
use crate::routes::verify::create_verification_id;

use super::error::{SignUpErrorResponse, SignUpErrorWrapper};

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*@[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*$").unwrap()
});

/// checks the validity of the signup info
pub fn validate_signup_info(
    con: &mut PgConnection,
    signup_info: &SignUpInfo,
    //verification_map: VerificationMap,
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

    // TODO: check in redis
    /*let verification_map = verification_map.read().unwrap();
    if verification_map
        .values()
        .any(|(user, _)| user.email == signup_info.email)
    {
        set_error_true!(sign_up_error, email_taken);
    }

    if verification_map
        .values()
        .any(|(user, _)| user.username == signup_info.username)
    {
        set_error_true!(sign_up_error, username_taken);
    }*/

    if is_email_in_db(con, &signup_info.email) {
        set_error_true!(sign_up_error, email_taken);
    }

    if is_username_in_db(con, &signup_info.username) {
        set_error_true!(sign_up_error, username_taken);
    }

    if sign_up_error.is_erroneous {
        return Err(SignUpErrorResponse::SignUpError(sign_up_error));
    }

    Ok(())
}

pub async fn add_new_user_to_redis(
    signup_info: SignUpInfo,
    redis_con: &mut redis::aio::Connection,
) -> Result<String, SignUpErrorResponse> {
    // create an (hopefully) unique verification id
    let id = create_verification_id(&signup_info);

    // save directly in mysql database
    // add 'verified' bool
    let new_user = NewUser::new(
        signup_info.email,
        signup_info.username,
        &signup_info.password,
    )
    .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    redis_con
        .set::<_, _, ()>(&id, new_user)
        .await
        .map_err(|_| SignUpErrorResponse::UserCreationError)?;

    // send verification email or afterwards, mind id return!
    // ...

    Ok(id)
}

#[cfg(test)]
mod tests {

    use diesel::{PgConnection, Connection};
    use somes_common_lib::SignUpInfo;

    use crate::establish_connection;
    use crate::model::NewUser;

    use crate::operations::user::insert_user;
    use crate::{routes::signup::error::SignUpErrorResponse};

    use super::validate_signup_info;

    #[test]
    fn test_validate_signup_info_short_password() {
        //let verification_map = VerificationMap::default();

        let signup_info = SignUpInfo {
            email: "test@test.at".to_string(),
            username: "test_name".to_string(),
            password: "supersicher".to_string(),
        };
        
        let con = &mut establish_connection();
        con.test_transaction::<_, (), _>(|con| {
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
            };
            Ok(())
        });
        
    }

    #[test]
    fn test_validate_signup_info_taken() {
        //let verification_map = VerificationMap::default();

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

        let con = &mut establish_connection();
        con.test_transaction::<_, (), _>(|con| {
            insert_user(con, &new_user).unwrap();

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
            Ok(())
        });
    }

    #[test]
    fn test_validate_signup_info_taken_verification_map() {
        //let verification_map = VerificationMap::default();

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

        // TODO
        /*verification_map
        .write()
        .unwrap()
        .insert("3".to_string(), (new_user, 1));*/

        let con = &mut establish_connection();
        con.test_transaction::<_, (), _>(|con| {
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
    
            let signup_info = SignUpInfo {
                email: "test1@test.at".to_string(),
                username: "test_name".to_string(),
                password: "supersicher".to_string(),
            };
    
            match validate_signup_info(con, &signup_info) {
                Ok(_) => panic!("Not possible, password is weak"),
                Err(err) => match err {
                    SignUpErrorResponse::UserCreationError => panic!(""),
                    SignUpErrorResponse::SignUpError(signup_err) => {
                        assert!(signup_err.insufficient_password);
                        assert!(!signup_err.email_taken);
                        assert!(signup_err.username_taken);
                    }
                },
            }
            Ok(())
        });
    }

    #[test]
    fn test_validate_signup_info_missing_entries() {
        // let verification_map = VerificationMap::default();

        let signup_info = SignUpInfo {
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
        };

        let con = &mut establish_connection();
        con.test_transaction::<_, (), _>(|con| {
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
            Ok(())
        });
        
    }
}
