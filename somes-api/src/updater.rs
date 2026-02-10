use std::time::Duration;

use dataservice::combx::{self, CombinedData};
use serde::{de::DeserializeOwned, Serialize};

use crate::set_json_cache_no_expire;

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

pub async fn update_caches<T: Serialize + DeserializeOwned + CombinedData>(
    client: &redis::Client,
    update_meilisearch_index_cb: impl AsyncFn(&[T]) -> combx::Result<()>,
) {
    let mut last_id = "$".to_string();
    let stream_id = T::INDEX.as_str();
    loop {
        let to_update = read_update_stream::<T>(stream_id, &mut last_id, client).await;

        match to_update {
            Ok(to_update) => {
                if let Err(e) = update_redis_entries(client, &to_update).await {
                    log::error!("Cannot update redis caches {stream_id}: {e:?}");
                }
                if let Err(e) = update_meilisearch_index_cb(&to_update).await {
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

pub async fn update_redis_entries<T: CombinedData + Serialize>(
    client: &redis::Client,
    to_update: &[T],
) -> redis::RedisResult<()> {
    let mut con = client.get_multiplexed_async_connection().await?;
    for data in to_update {
        let key = format!("{}/{}", T::INDEX.as_str(), data.id());
        let _ = set_json_cache_no_expire(&mut con, &key, data);
    }

    Ok(())
}
