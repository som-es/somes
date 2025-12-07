use axum::Json;
use somes_common_lib::{Delegate, FullMandate};
use crate::{PgPoolConnection, routes::DelegatesErrorResponse};

#[utoipa::path(
    get,
    path = "/delegates",
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<Delegate>]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn active_delegates_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    Ok(sqlx::query_as!(
        Delegate,
        "
SELECT
    * from delegates_with_mandates
WHERE
    is_active
    -- return only delegates with at least on 'is_nr' mandate
    AND EXISTS (
      SELECT 1
      FROM unnest(\"active_mandates: Vec<FullMandate>\") am
      WHERE am.is_nr and am.end_date IS NULL
  );
        "
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .unwrap())
}
