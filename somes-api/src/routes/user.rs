mod error;

use axum::Json;

use crate::{
    jwt::Claims, model::User, operations::user::get_user_from_db_by_id,
    routes::user::error::UserErrorResponse, SomesDbConnection,
};

pub async fn user(
    claims: Claims,
    SomesDbConnection(con): SomesDbConnection,
) -> Result<Json<User>, UserErrorResponse> {
    con.interact(move |con| {
        get_user_from_db_by_id(con, claims.id)
            .map(Json)
            .ok_or(UserErrorResponse::InvalidUser)
    })
    .await
    .map_err(|_| UserErrorResponse::InteractionFailed)?
}
