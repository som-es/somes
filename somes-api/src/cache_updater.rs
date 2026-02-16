mod update_gov_proposals;
mod update_vote_results;

use std::time::Duration;

use dataservice::combx::{self, CombinedData};
use redis::aio::MultiplexedConnection;
use serde::{de::DeserializeOwned, Serialize};

use crate::{meilisearch::update_time, set_json_cache_no_expire};

pub fn update_caches(
    redis_client: &redis::Client,
    dataservice_sqlx_pool: &sqlx::Pool<sqlx::Postgres>,
    meilisearch_client: &meilisearch_sdk::client::Client,
) {
    let inner_redis_client = redis_client.clone();
    let inner_pool = dataservice_sqlx_pool.clone();
    let inner_meilisearch_client = meilisearch_client.clone();

    tokio::task::spawn(update_vote_results::update_cache_vote_results(
        inner_redis_client,
        inner_pool,
        inner_meilisearch_client,
    ));
}

pub(crate) async fn read_update_stream<T: DeserializeOwned>(
    stream_id: &str,
    last_id: &mut String,
    client: &redis::Client,
) -> dataservice::combx::Result<Vec<T>> {
    let mut con = client.get_multiplexed_async_connection().await?;

    let reply: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = redis::cmd("XREAD")
        .arg("BLOCK")
        .arg(5000)
        .arg("STREAMS")
        .arg(stream_id)
        .arg(&*last_id)
        .query_async(&mut con)
        .await
        .unwrap_or_default();

    let mut out = Vec::new();

    for (_stream, entries) in reply {
        for (id, fields) in entries {
            for (k, v) in fields {
                if k == "data" {
                    out.push(serde_json::from_str::<T>(&v)?);
                }
            }
            *last_id = id;
        }
    }

    Ok(out)
}

pub async fn update_cache_for_index<T: Serialize + DeserializeOwned + CombinedData>(
    client: &redis::Client,
    intercept_and_update_cb: impl AsyncFn(&T) -> combx::Result<T>,
    notify_dependencies: impl AsyncFn(&redis::Client, &T) -> combx::Result<()>,
    update_meilisearch_index_cb: impl AsyncFn(Vec<T>) -> combx::Result<()>,
) {
    let mut last_id = "$".to_string();
    let stream_id = T::INDEX.as_str();
    loop {
        let to_update = read_update_stream::<T>(stream_id, &mut last_id, client).await;

        match to_update {
            Ok(mut to_update) => {
                if let Err(e) = update_redis_entries(
                    &intercept_and_update_cb,
                    &notify_dependencies,
                    client,
                    &mut to_update,
                )
                .await
                {
                    log::error!("Cannot update redis caches {stream_id}: {e:?}");
                }
                // probably still better to update periodically (or add dependencies) - custom function, e.g. gov proposal is missing delegate
                if let Err(e) = update_meilisearch_index_cb(to_update).await {
                    log::error!("Cannot update meilisearch index {stream_id}: {e:?}");
                }
            }
            Err(e) => {
                log::error!("Cannot read update stream for {stream_id}: {e:?}");
                tokio::time::sleep(Duration::from_millis(30)).await;
                continue;
            }
        }
    }
}

pub async fn update_meilisearch_index<T: CombinedData + serde::Serialize + Send + Sync>(
    data: &[T],
    meilisearch_client: &meilisearch_sdk::client::Client,
    redis_con: &mut MultiplexedConnection,
) -> combx::Result<()> {
    meilisearch_client
        .index(T::INDEX.as_str())
        .add_documents_in_batches(&data, Some(3000), Some(T::PRIMARY_KEY))
        .await?;

    update_time::update_update_time_of_index(redis_con, &T::INDEX).await?;
    Ok(())
}

pub async fn update_redis_entries<T: CombinedData + Serialize>(
    intercept_and_update_cb: &impl AsyncFn(&T) -> combx::Result<T>,
    notify_dependencies: &impl AsyncFn(&redis::Client, &T) -> combx::Result<()>,
    client: &redis::Client,
    to_update: &mut [T],
) -> combx::Result<()> {
    let mut con = client.get_multiplexed_async_connection().await?;
    for data in to_update {
        // e.g. vote result fetch for gov proposals, meilisearch helper for vote results
        *data = intercept_and_update_cb(data).await?;
        notify_dependencies(client, data).await?;
        let key = format!("{}/{}", T::INDEX.as_str(), data.id());
        let _ = set_json_cache_no_expire(&mut con, &key, data);
    }

    Ok(())
}
