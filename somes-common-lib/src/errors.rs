use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Default, ToSchema)]
pub struct SignUpError {
    pub missing_username: bool,
    pub missing_password: bool,
    pub missing_email: bool,
    pub username_taken: bool,
    pub email_taken: bool,
    pub invalid_email: bool,
    pub invalid_otp: bool,
    pub insufficient_password: bool,
    pub is_erroneous: bool,
}

#[macro_export]
macro_rules! set_error_true {
    ($error:ident, $error_type:ident) => {
        $error.$error_type = true;
        $error.is_erroneous = true;
    };
}
