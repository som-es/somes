use dataservice::combx::{Delegate, FullMandate};
use sqlx::PgPool;

use crate::{
    update_cache_for_index, update_meilisearch_index,
};

pub async fn update_cache_delegates(
    redis_client: redis::Client,
    pool: PgPool,
    meilisearch_client: meilisearch_sdk::client::Client,
) {
    let meilisearch_client = meilisearch_client.clone();
    let inner_redis_client = redis_client.clone();
    let update_meilisearch_index = move |delegates: Vec<dataservice::combx::Delegate>| {
        let meilisearch_client = meilisearch_client.clone();
        let inner_redis_client = inner_redis_client.clone();

        async move {
            let mut redis_con = inner_redis_client
                .get_multiplexed_async_connection()
                .await?;
            // delegate_by_id_sqlx(*id, &inner_pool, &mut redis_con).await?
            update_meilisearch_index(&delegates, &meilisearch_client, &mut redis_con).await?;
            Ok(())
        }
    };

    let inner_pool = pool.clone();
    let intercept_and_update_cb = move |data: common_scrapes::Delegate| {
        let pool = inner_pool.clone();
        let delegate_id = data.id as i32;
        async move {
            // could use patching approach in the future
            // https://github.com/yanganto/struct-patch/
            let delegate = sqlx::query_as!(
                Delegate,
                "
                SELECT
                    * from delegates_with_mandates d
                WHERE
                d.id = $1;
            ",
                delegate_id
            )
            .fetch_one(&pool)
            .await?;

            Ok(delegate)
        }
    };

    let notify_dependencies =
        move |_redis_client: &redis::Client, _data: &dataservice::combx::Delegate| async move { Ok(()) };

    update_cache_for_index::<common_scrapes::Delegate, dataservice::combx::Delegate>(
        &redis_client,
        intercept_and_update_cb,
        notify_dependencies,
        update_meilisearch_index,
    )
    .await;
}
