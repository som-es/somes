pub mod errors;
pub mod password;
mod routes;
mod structs;
pub mod time;

use dotenv_codegen::dotenv;
use once_cell::sync::Lazy;
pub use routes::*;
pub use structs::*;

pub static VERIFY_ID_INVALID_HOURS: Lazy<usize> = Lazy::new(|| {
    let min_password_len_str: &str = dotenv!("VERIFY_ID_INVALID_HOURS");
    min_password_len_str
        .parse::<usize>()
        .expect("The value provided for VERIFY_ID_INVALID_HOURS in the .env is not a number!")
});
