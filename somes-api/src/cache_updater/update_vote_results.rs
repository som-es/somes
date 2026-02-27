use dataservice::{
    combx::{self, GovProposal, OptionalVoteResult},
    notify_update_streamed,
};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{routes::fetch_vote_result_by_id, update_cache_for_index, update_meilisearch_index};

pub async fn update_cache_vote_results(
    redis_client: redis::Client,
    pool: PgPool,
    meilisearch_client: meilisearch_sdk::client::Client,
) {
    let meilisearch_client = meilisearch_client.clone();
    let inner_redis_client = redis_client.clone();
    let update_meilisearch_index = move |mut vote_results: Vec<
        dataservice::combx::OptionalVoteResult,
    >| {
        let meilisearch_client = meilisearch_client.clone();
        let redis_client = inner_redis_client.clone();
        for vote_result in vote_results.iter_mut() {
            if let Some(meilisearch_helper) = vote_result.meilisearch_helper.as_mut() {
                meilisearch_helper.votes = vote_result
                    .votes
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|vote| format!("{}{:?}", vote.party, vote.infavor))
                    .collect();
            }
        }

        async move {
            let mut redis_con = redis_client.get_multiplexed_async_connection().await?;
            update_meilisearch_index(&vote_results, &meilisearch_client, &mut redis_con).await?;
            Ok(())
        }
    };

    let inner_pool = pool.clone();
    let intercept_and_update_cb = move |data: dataservice::combx::OptionalVoteResult| {
        let pool = inner_pool.clone();
        let id = data.id;
        async move {
            let id = id.ok_or(sqlx::Error::RowNotFound)?;

            // could use patching approach in the future
            // https://github.com/yanganto/struct-patch/
            let data = fetch_vote_result_by_id(&pool, id).await?;

            Ok(data.ok_or(sqlx::Error::RowNotFound)?)
        }
    };

    let inner_pool = pool.clone();
    let notify_dependencies =
        move |mut redis_con: MultiplexedConnection,
              data: &dataservice::combx::OptionalVoteResult| {
            let pool = inner_pool.clone();
            let legis_init = data.legislative_initiative.as_ref().unwrap();
            let gp = legis_init.gp.clone();
            let ityp = legis_init.ityp.clone();
            let inr = legis_init.inr;
            async move {
                let ministerial_props = sqlx::query!("
                select id from ministrial_proposals where legis_init_gp = $1 and legis_init_inr = $2 and legis_init_ityp = $3",
                gp,
                inr,
                ityp
            ).fetch_all(&pool).await?;

                for ministerial_prop in ministerial_props {
                    let data = GovProposal {
                        id: Some(ministerial_prop.id),
                        ..Default::default()
                    };
                    notify_update_streamed(combx::Index::GovProposals, &data, &mut redis_con)
                        .await?;
                }
                Ok(())
            }
        };

    update_cache_for_index::<OptionalVoteResult, OptionalVoteResult>(
        &redis_client,
        intercept_and_update_cb,
        notify_dependencies,
        update_meilisearch_index,
    )
    .await;
}
