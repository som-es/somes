use crate::{db::create_db_if_not_exists, jwt::KEYS};

pub fn initial_startup(db_path: &'static str, sql_db_path: &'static str) {
    // access keys for lazy eval -> check if JWT_SECRET is set
    let _ = &KEYS.decoding;

    // production database
    if let Err(e) = create_db_if_not_exists(db_path, sql_db_path) {
        panic!("Error: {e}");
    }
}
