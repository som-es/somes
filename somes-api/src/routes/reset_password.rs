use axum::Json;
use jsonwebtoken::{encode, Header};
use somes_common_lib::{time::timestamp_secs, NewPasswordInfo, ResetPasswordInfo};

use crate::{
    email::{send_mail, MAILER},
    establish_connection,
    hash::hash_password,
    jwt::{Claims, KEYS},
    operations::user::{get_user_from_db, update_password_hash_at},
};

pub async fn send_reset_password_request(
    // claims: Claims,
    Json(reset_password_info): Json<ResetPasswordInfo>,
    // update Error
) -> Result<Json<()>, ()> {
    let con = &mut establish_connection();
    let Some(user) = get_user_from_db(con,&reset_password_info.username_or_email, &reset_password_info.username_or_email) else {
        return Ok(Json(()))
    };

    let claims = Claims {
        sub: user.username.to_string(),
        company: "".into(),
        exp: (timestamp_secs() + 60 * 15) as usize,
    };

    // Create the authorization token
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();

    // send mail
    if let Err(e) = send_mail(&MAILER, &user.email, "Password reset request for your somes account", format!("Please click on the following link to reset your password: https://somes.at/reset_password?token={access_token}")) {
        log::info!("Error sending mail: {e}", )
    }
    Ok(Json(()))
}

pub async fn reset_password(
    claims: Claims,
    Json(new_password_info): Json<NewPasswordInfo>,
    // update Error
) -> Result<Json<()>, ()> {
    let new_password_hash = hash_password(&new_password_info.password).unwrap();
    let username = claims.sub;

    let mut con = establish_connection();

    // update password in db
    update_password_hash_at(&mut con, &username, &new_password_hash);

    Ok(Json(()))
}
