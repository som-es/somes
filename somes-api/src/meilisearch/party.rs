use futures::FutureExt;

pub async fn party_of_delegate_at_time(
    date: chrono::NaiveDate,
    delegate_id: i32,
    pool: &sqlx::PgPool,
) -> sqlx::Result<Option<String>> {
    sqlx::query_scalar!(
        "
        SELECT party
        FROM mandates m
        where is_nr and delegate_id = $2 and start_date <= $1::date AND COALESCE(end_date, $1::date) >= $1::date
        LIMIT 1",
        date,
        delegate_id
    )
    .fetch_one(pool)
    .await
}

pub async fn party_of_delegates_at_time(
    date: chrono::NaiveDate,
    delegate_ids: &[i32],
    pool: &sqlx::PgPool,
) -> Vec<String> {
    let start = tokio::time::Instant::now();
    let data = futures::future::join_all(delegate_ids.iter().map(|delegate_id| async {
        party_of_delegate_at_time(date, *delegate_id, pool)
            .await
            .ok()
            .flatten()
    }))
    .map(|parties| parties.into_iter().flatten().collect::<Vec<_>>())
    .await;

    log::info!("party selection duration: {:?}", start.elapsed());

    data
}
