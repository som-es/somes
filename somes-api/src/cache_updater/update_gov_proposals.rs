use dataservice::combx::OptionalGovProposal;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    routes::{delegate_by_id_sqlx, get_gov_proposal_sqlx, GovProposalDelegate},
    update_cache_for_index, update_meilisearch_index,
};

pub async fn update_cache_gov_proposals(
    redis_client: redis::Client,
    pool: PgPool,
    meilisearch_client: meilisearch_sdk::client::Client,
) {
    let meilisearch_client = meilisearch_client.clone();
    let inner_redis_client = redis_client.clone();
    let inner_pool = pool.clone();
    let update_meilisearch_index = move |gov_proposals: Vec<
        dataservice::combx::OptionalGovProposal,
    >| {
        let meilisearch_client = meilisearch_client.clone();
        let mut gov_proposal_delegates =
            Vec::<GovProposalDelegate>::with_capacity(gov_proposals.len());
        let inner_redis_client = inner_redis_client.clone();
        let inner_pool = inner_pool.clone();

        async move {
            let mut redis_con = inner_redis_client
                .get_multiplexed_async_connection()
                .await?;
            for gov_proposal in gov_proposals {
                let delegate = match gov_proposal
                    .ministerial_issuers
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .next()
                {
                    Some(id) => Some(delegate_by_id_sqlx(*id, &inner_pool, &mut redis_con).await?),
                    None => None,
                };

                gov_proposal_delegates.push(GovProposalDelegate {
                    gov_proposal,
                    delegate,
                })
            }

            update_meilisearch_index(&gov_proposal_delegates, &meilisearch_client, &mut redis_con)
                .await?;
            Ok(())
        }
    };

    let inner_pool = pool.clone();
    let intercept_and_update_cb = move |data: dataservice::combx::OptionalGovProposal| {
        let pool = inner_pool.clone();
        let id = data.id;
        async move {
            let id = id.ok_or(sqlx::Error::RowNotFound)?;

            // could use patching approach in the future
            // https://github.com/yanganto/struct-patch/
            let data = get_gov_proposal_sqlx(&pool, id).await?;
            Ok(data)
        }
    };

    let notify_dependencies =
        move |_redis_client: MultiplexedConnection,
              _data: &dataservice::combx::OptionalGovProposal| async move { Ok(()) };

    update_cache_for_index::<OptionalGovProposal, OptionalGovProposal>(
        &redis_client,
        intercept_and_update_cb,
        notify_dependencies,
        update_meilisearch_index,
    )
    .await;
}
