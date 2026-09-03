use std::time::Duration;

use combx::{Index, Parliament};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use super::decrees::create_or_update_decrees_meilisearch_index;
use super::delegate_questions::update_delegate_questions_meilisearch_index;
use super::delegates::update_delegates_meilisearch_index;
use super::gov_props::create_or_update_gov_props_meilisearch_index;
use super::vote_results::update_vote_result_meilisearch_index;
use crate::AppState;
use crate::IS_PROD;
use crate::routes::all_votes_from_legis_init;

const VOTE_RESULT_REFRESH_INTERVAL: Duration = Duration::from_secs(1900);
const INDEX_REFRESH_INTERVAL: Duration = Duration::from_secs(1000);

#[derive(Clone, Copy)]
pub enum IndexRefresher {
    VoteResults,
    GovProposals,
    Decrees,
    Delegates,
    DelegateQuestions,
}

pub const REFRESHERS: [IndexRefresher; 5] = [
    IndexRefresher::Delegates,
    IndexRefresher::Decrees,
    IndexRefresher::GovProposals,
    IndexRefresher::VoteResults,
    IndexRefresher::DelegateQuestions,
];

impl IndexRefresher {
    pub fn index(self) -> Index {
        match self {
            Self::VoteResults => Index::VoteResults,
            Self::GovProposals => Index::GovProposals,
            Self::Decrees => Index::Decrees,
            Self::Delegates => Index::Delegates,
            Self::DelegateQuestions => Index::DelegateQuestions,
        }
    }

    pub fn refresh_interval(self) -> Duration {
        match self {
            Self::VoteResults => VOTE_RESULT_REFRESH_INTERVAL,
            Self::GovProposals | Self::Decrees | Self::Delegates | Self::DelegateQuestions => {
                INDEX_REFRESH_INTERVAL
            }
        }
    }

    pub fn keeps_running_in_prod(self) -> bool {
        matches!(self, Self::Decrees | Self::DelegateQuestions)
    }

    pub async fn update(
        self,
        parliament: Parliament,
        pg_pool: &sqlx::Pool<sqlx::Postgres>,
        redis_con: &mut ConnectionManager,
        client: &Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::VoteResults => {
                update_vote_result_meilisearch_index(
                    parliament,
                    pg_pool,
                    redis_con,
                    client,
                    all_votes_from_legis_init,
                )
                .await
            }
            Self::GovProposals => {
                create_or_update_gov_props_meilisearch_index(parliament, pg_pool, redis_con, client)
                    .await
            }
            Self::Decrees => {
                create_or_update_decrees_meilisearch_index(parliament, pg_pool, redis_con, client)
                    .await
            }
            Self::Delegates => {
                update_delegates_meilisearch_index(parliament, pg_pool, redis_con, client).await
            }
            Self::DelegateQuestions => {
                update_delegate_questions_meilisearch_index(parliament, pg_pool, redis_con, client)
                    .await
            }
        }
    }
}

fn spawn_parliament_index_refreshers(
    parliament: Parliament,
    redis_con: &ConnectionManager,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    meilisearch_client: &Client,
    prod_wait_handles: &mut Vec<JoinHandle<()>>,
) {
    for refresher in REFRESHERS {
        let pg_pool = pg_pool.clone();
        let mut redis_con = redis_con.clone();
        let meilisearch_client = meilisearch_client.clone();

        let handle = tokio::task::spawn(async move {
            let index = refresher.index().uid(parliament);

            loop {
                if let Err(error) = refresher
                    .update(parliament, &pg_pool, &mut redis_con, &meilisearch_client)
                    .await
                {
                    log::warn!(
                        "Could not update meilisearch index {index} ({parliament}): {error:?}"
                    );
                }

                if *IS_PROD && !refresher.keeps_running_in_prod() {
                    break;
                }

                let refresh_interval = refresher.refresh_interval();
                log::info!("Sleeping {refresh_interval:?} before refreshing {index}");
                sleep(refresh_interval).await;
            }
        });

        if !refresher.keeps_running_in_prod() {
            prod_wait_handles.push(handle);
        }
    }
}

pub async fn update_meilisearch_indices(app_state: &AppState) {
    let mut prod_wait_handles = vec![];

    spawn_parliament_index_refreshers(
        Parliament::At,
        &app_state.redis.connection,
        &app_state.dataservice_sqlx_pool,
        &app_state.meilisearch_client,
        &mut prod_wait_handles,
    );

    spawn_parliament_index_refreshers(
        Parliament::Eu,
        &app_state.eu_redis.connection,
        &app_state.eu_dataservice_sqlx_pool,
        &app_state.meilisearch_client,
        &mut prod_wait_handles,
    );

    if *IS_PROD {
        for handle in prod_wait_handles {
            if let Err(e) = handle.await {
                log::error!("Could not force update for cache: {e:}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use combx::Index;

    use super::{INDEX_REFRESH_INTERVAL, IndexRefresher, REFRESHERS, VOTE_RESULT_REFRESH_INTERVAL};

    fn registered_indices() -> Vec<Index> {
        REFRESHERS
            .iter()
            .map(|refresher| refresher.index())
            .collect()
    }

    #[test]
    fn registry_registers_every_index_exactly_once() {
        let expected = [
            Index::VoteResults,
            Index::GovProposals,
            Index::Decrees,
            Index::Delegates,
            Index::DelegateQuestions,
        ];
        let registered = registered_indices();

        assert_eq!(registered.len(), expected.len());
        for index in expected {
            assert_eq!(
                registered
                    .iter()
                    .filter(|registered| **registered == index)
                    .count(),
                1,
                "{index} must be registered exactly once"
            );
        }
    }

    #[test]
    fn only_decrees_and_delegate_questions_keep_running_in_prod() {
        let keeps_running = REFRESHERS
            .iter()
            .filter(|refresher| refresher.keeps_running_in_prod())
            .map(|refresher| refresher.index())
            .collect::<Vec<Index>>();

        assert_eq!(
            keeps_running,
            vec![Index::Decrees, Index::DelegateQuestions]
        );
    }

    #[test]
    fn vote_results_are_refreshed_less_often_than_the_other_indices() {
        assert_eq!(
            IndexRefresher::VoteResults.refresh_interval(),
            VOTE_RESULT_REFRESH_INTERVAL
        );
        assert_eq!(VOTE_RESULT_REFRESH_INTERVAL, Duration::from_secs(1900));

        for refresher in REFRESHERS {
            if !matches!(refresher, IndexRefresher::VoteResults) {
                assert_eq!(refresher.refresh_interval(), INDEX_REFRESH_INTERVAL);
            }
        }
        assert_eq!(INDEX_REFRESH_INTERVAL, Duration::from_secs(1000));
    }
}
