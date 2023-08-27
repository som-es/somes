use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// 'ResetPasswordInfo' is used to send a reset password request to the server.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct ResetPasswordInfo {
    pub username_or_email: String,
}

/// 'NewPasswordInfo' is used to transmit a new password to the server.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct NewPasswordInfo {
    pub password: String,
}

/// `LoginInfo` is sent by the client and received by the server at login of a user.
#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_or_email: String,
    pub password: String,
}

/// `SignUpInfo` is sent by the client and received by the server at registering a new user.
#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct SignUpInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct JWTInfo {
    pub access_token: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone, Copy)]
pub struct Unit;

#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct VerificationIDInfo {
    pub id: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct SaveEmailInfo {
    pub email: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct AskQuestion {
    pub delegate_id: i32,
    pub title: String,
    pub body: String,
}
