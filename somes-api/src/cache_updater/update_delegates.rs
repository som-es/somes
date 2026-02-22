use dataservice::{combx::{self, Delegate, FullMandate, GovProposal}, notify_update_streamed};
use sqlx::PgPool;

use crate::{update_cache_for_index, update_meilisearch_index};

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

    let inner_pool = pool.clone();
    let notify_dependencies =
        move |redis_client: &redis::Client, data: &dataservice::combx::Delegate| {
            let pool = inner_pool.clone();
            let id = data.id;
            let redis_client = redis_client.clone();
            async move { 
                let id = id.ok_or(sqlx::Error::RowNotFound)?;
                let ministerial_prop_id = sqlx::query_scalar!(
                    "select ministrial_proposal_id from ministrial_issuer where delegate_id = $1 limit 1",
                    id
                ).fetch_optional(&pool).await?;

                if let Some(ministerial_prop_id) = ministerial_prop_id {
                    let data = GovProposal {
                        id: Some(ministerial_prop_id),
                        ..Default::default()
                    };
                    notify_update_streamed(combx::Index::GovProposals, &data, &redis_client).await?;
                }
                Ok(()) 
            }
        };

    update_cache_for_index::<common_scrapes::Delegate, dataservice::combx::Delegate>(
        &redis_client,
        intercept_and_update_cb,
        notify_dependencies,
        update_meilisearch_index,
    )
    .await;
}
