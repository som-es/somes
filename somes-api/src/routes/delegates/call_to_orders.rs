use sqlx::PgPool;

pub(crate) async fn extract_call_to_orders_by_delegate(
    delegate_id: i32,
    pg_pool: &PgPool,
) -> sqlx::Result<Vec<i32>> {
    sqlx::query_scalar!(
        "
        select plenar_id from call_to_order where receiver_id = $1 
    ",
        delegate_id
    )
    .fetch_all(pg_pool)
    .await
}
