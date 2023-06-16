use serde::{Deserialize, Serialize};

/// 'ResetPasswordInfo' is used to send a reset password request to the server.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ResetPasswordInfo {
    pub username_or_email: String,
}

/// 'NewPasswordInfo' is used to transmit a new password to the server.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct NewPasswordInfo {
    pub password: String,
}

/// `LoginInfo` is sent by the client and received by the server at login of a user.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_or_email: String,
    pub password: String,
}

/// `SignUpInfo` is sent by the client and received by the server at registering a new user.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SignUpInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct JWTInfo {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Unit;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct VerificationIDInfo {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SaveEmailInfo {
    email: String,
}
