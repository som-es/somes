mod update_delegates;
mod update_gov_proposals;
mod update_vote_results;

use std::time::Duration;

use dataservice::combx::{self, CombinedData};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

use crate::{meilisearch::update_time, set_json_cache_no_expire};

pub(crate) fn update_caches(
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

    let inner_redis_client = redis_client.clone();
    let inner_pool = dataservice_sqlx_pool.clone();
    let inner_meilisearch_client = meilisearch_client.clone();

    tokio::task::spawn(update_gov_proposals::update_cache_gov_proposals(
        inner_redis_client,
        inner_pool,
        inner_meilisearch_client,
    ));

    let inner_redis_client = redis_client.clone();
    let inner_pool = dataservice_sqlx_pool.clone();
    let inner_meilisearch_client = meilisearch_client.clone();

    tokio::task::spawn(update_delegates::update_cache_delegates(
        inner_redis_client,
        inner_pool,
        inner_meilisearch_client,
    ));
}

pub(crate) async fn read_update_stream<T: DeserializeOwned>(
    stream_id: &str,
    last_id: &mut String,
    con: &mut MultiplexedConnection,
) -> dataservice::combx::Result<Vec<T>> {
    // const BLOCK_MS: usize = 2000;

    let key = format!("{stream_id}/last_id");
    *last_id = con.get::<_, String>(&key).await.unwrap_or("0".into());

    let reply: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = redis::cmd("XREAD")
        .arg("STREAMS")
        .arg(stream_id)
        .arg(&*last_id)
        .query_async(con)
        .await
        .unwrap_or_default();

    log::info!("reply len: {}", reply.len());
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

    con.set::<_, &str, ()>(&key, last_id.as_str()).await?;
    Ok(out)
}

pub async fn update_cache_for_index<
    T: Serialize + DeserializeOwned,
    I: Serialize + DeserializeOwned + CombinedData,
>(
    client: &redis::Client,
    intercept_and_update_cb: impl AsyncFn(T) -> combx::Result<I>,
    notify_dependencies: impl AsyncFn(MultiplexedConnection, &I) -> combx::Result<()>,
    update_meilisearch_index_cb: impl AsyncFn(Vec<I>) -> combx::Result<()>,
) {
    // let mut last_id = "$".to_string();
    let mut last_id = "0".to_string();
    let stream_id = I::INDEX.as_str();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    loop {
        let to_update = read_update_stream::<T>(stream_id, &mut last_id, &mut con).await;

        match to_update {
            Ok(to_update) => {
                log::info!("({stream_id}) received {} entries", to_update.len());
                // e.g. vote result fetch for gov proposals, meilisearch helper for vote results
                let to_update = convert_entries(to_update, &intercept_and_update_cb).await;
                if let Err(e) =
                    update_redis_entries(&notify_dependencies, &mut con, &to_update).await
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
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
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

pub async fn convert_entries<T, I>(
    entries: Vec<T>,
    intercept_and_update_cb: &impl AsyncFn(T) -> combx::Result<I>,
) -> Vec<I> {
    let mut converted_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        let Ok(converted_entry) = intercept_and_update_cb(entry).await else {
            continue;
        };
        converted_entries.push(converted_entry)
    }
    converted_entries
}

pub async fn update_redis_entries<T: CombinedData + Serialize>(
    notify_dependencies: &impl AsyncFn(MultiplexedConnection, &T) -> combx::Result<()>,
    con: &mut MultiplexedConnection,
    to_update: &[T],
) -> combx::Result<()> {
    for data in to_update {
        notify_dependencies(con.clone(), data).await?;
        let key = format!("{}/{}", T::INDEX.as_str(), data.id());
        let _ = set_json_cache_no_expire(con, &key, data);
    }

    Ok(())
}
