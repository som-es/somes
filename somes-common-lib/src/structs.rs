
/// `LoginInfo` is sent by the client and received by the server at login of a user.
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

/// `RegisterInfo` is sent by the client and received by the server at registering a new user.
pub struct RegisterInfo {
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
pub struct UserInfo {
    pub username: String,
}