use std::time::Duration;

use meilisearch_sdk::{
    client::Client,
    errors::{Error, ErrorCode, MeilisearchError},
    settings::Settings,
};

const MEILISEARCH_TASK_TIMEOUT: Duration = Duration::from_secs(600);
const MEILISEARCH_TASK_POLL_INTERVAL: Duration = Duration::from_millis(200);

pub(crate) async fn index_exists(client: &Client, uid: &str) -> Result<bool, Error> {
    match client.get_index(uid).await {
        Ok(_) => Ok(true),
        Err(Error::Meilisearch(MeilisearchError {
            error_code: ErrorCode::IndexNotFound,
            ..
        })) => Ok(false),
        Err(e) => Err(e), // network error, auth error, etc.
    }
}

pub(crate) async fn rebuild_index_via_swap<T: serde::Serialize + Send + Sync>(
    client: &Client,
    index: &str,
    settings: &Settings,
    documents: &[T],
    primary_key: Option<&str>,
    batch_size: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let swap_index = format!("{index}_swap");

    if let Ok(exists) = index_exists(client, index).await {
        if !exists {
            let task = client.create_index(index, None).await?;
            task.wait_for_completion(client, None, None).await?;
        }
    }

    client
        .index(&swap_index)
        .set_settings(settings)
        .await?
        .wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;

    let upload_tasks = client
        .index(&swap_index)
        .add_documents_in_batches(documents, batch_size, primary_key)
        .await?;

    for task in upload_tasks {
        task.wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;
    }

    client
        .swap_indexes(&[meilisearch_sdk::client::SwapIndexes {
            indexes: (index.to_string(), swap_index.clone()),
            rename: None,
        }])
        .await?
        .wait_for_completion(
            client,
            Some(MEILISEARCH_TASK_POLL_INTERVAL),
            Some(MEILISEARCH_TASK_TIMEOUT),
        )
        .await?;

    client.delete_index(&swap_index).await?;

    Ok(())
}
