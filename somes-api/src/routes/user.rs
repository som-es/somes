mod error;

use axum::Json;
use somes_common_lib::UserInfo;

use crate::{
    jwt::Claims, operations::user::get_user_from_db_by_id,
    routes::user::error::UserErrorResponse, SomesDbConnection,
};

pub async fn user(
    claims: Claims,
    SomesDbConnection(con): SomesDbConnection,
) -> Result<Json<UserInfo>, UserErrorResponse> {
    con.interact(move |con| {
        get_user_from_db_by_id(con, claims.id)
            .map(|user| UserInfo { username: user.username, email: user.email })
            .map(Json)
            .ok_or(UserErrorResponse::InvalidUser)
    })
    .await
    .map_err(|_| UserErrorResponse::InteractionFailed)?
}
