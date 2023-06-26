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
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AskQuestion {
    /// mind keeping this id private
    pub issuer_id: i32,
    pub delegate_id: i32,
    pub text: String,
    pub body: String,
}
