use somes_common_lib::CallToOrder;
use sqlx::PgPool;

pub(crate) async fn extract_call_to_orders_by_delegate(
    delegate_id: i32,
    pg_pool: &PgPool,
) -> sqlx::Result<Vec<CallToOrder>> {
    sqlx::query_as!(CallToOrder,
        "
        select 
            pi.inr, 
            pi.legislative_period as gp, 
            pi.raw_data_created_at as date, 
            plenar_id as plenary_session_id 
        from call_to_order cto inner join plenar_infos pi on cto.plenar_id = pi.id where receiver_id = $1 
    ",
        delegate_id
    )
    .fetch_all(pg_pool)
    .await
}
