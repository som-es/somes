mod error;
use std::collections::HashMap;

use crate::{get_json_cache, GenericError, PgPoolConnection, RedisConnection};
use axum::{extract::Query, Json};
use common_scrapes::Party;
use dataservice::combx::DbVote;
pub use error::*;
use serde::{Deserialize, Serialize};
use somes_common_lib::LegisPeriodGp;
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/parties",
    responses(
        // (status = 200, description = "Returned parties successfully.", body = [Vec<Party>]),
        // (status = 400, description = "Invalid request", body = [PartiesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [PartiesErrorResponse])
    )
)]
pub async fn parties_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    Ok(
        dataservice::combx::with_data::all_parties_by_most_recent_gp(&pg)
            .await
            .map(Json)?,
    )
}

pub async fn parties_per_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<HashMap<String, Vec<Party>>>, PartiesErrorResponse> {
    Ok(dataservice::combx::with_data::all_parties_per_gp(&pg)
        .await
        .map(Json)?)
}

pub async fn parties_at_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(legis_period): Query<LegisPeriodGp>,
) -> Result<Json<Vec<Party>>, PartiesErrorResponse> {
    Ok(
        dataservice::combx::with_data::all_parties_at_gp(&pg, &legis_period.gp)
            .await
            .map(Json)?,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartyStates {
    pub opposition_parties: Vec<Party>,
    pub coalition_parties: Vec<Party>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LegislativeInitiativeWithVotes {
    pub id: Option<i32>,
    pub gp: Option<String>,
    pub votes: Option<Vec<DbVote>>,
}

pub async fn coalition_parties_per_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(redis_con): RedisConnection,
) -> Result<Json<HashMap<String, PartyStates>>, GenericError> {
    Ok(determine_party_states_per_gp(&pg, redis_con)
        .await
        .map(Json)?)
}

pub async fn initiatives_with_votes_cached(
    pool: &PgPool,
    redis_con: &mut redis::aio::MultiplexedConnection,
) -> Result<Vec<LegislativeInitiativeWithVotes>, sqlx::Error> {
    if let Some(initiatives_with_votes) = get_json_cache(redis_con, "votes_only_legis_inits").await
    {
        return Ok(initiatives_with_votes);
    }

    initiatives_with_votes(pool).await
}

pub async fn initiatives_with_votes(
    pool: &PgPool,
) -> Result<Vec<LegislativeInitiativeWithVotes>, sqlx::Error> {
    let initiatives = sqlx::query_as!(
        LegislativeInitiativeWithVotes,
        r#"
            select * from legislative_initiatives_with_votes
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(initiatives)
}

pub async fn determine_party_states_per_gp(
    pool: &PgPool,
    mut redis_con: redis::aio::MultiplexedConnection,
) -> crate::Result<HashMap<String, PartyStates>> {
    let legis_inits_with_votes = initiatives_with_votes_cached(pool, &mut redis_con)
        .await
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    let parties_per_gp = dataservice::combx::with_data::all_parties_per_gp(&pool)
        .await
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;
    let coalition_parties_per_gp =
        determine_coalition_from_voting_behaviour_per_gp(&legis_inits_with_votes);

    let mut party_states_per_gp = HashMap::new();

    for (gp, coalition_parties) in coalition_parties_per_gp {
        let parties = &parties_per_gp[&gp];
        let opposition_parties = parties
            .iter()
            .filter(|party| !coalition_parties.contains(&party.name))
            .cloned()
            .collect::<Vec<_>>();

        let coalition_parties = parties
            .iter()
            .filter(|party| coalition_parties.contains(&party.name))
            .cloned()
            .collect::<Vec<_>>();

        party_states_per_gp.insert(
            gp,
            PartyStates {
                opposition_parties,
                coalition_parties,
            },
        );
    }

    Ok(party_states_per_gp)
}

pub type Gp = String;

pub fn determine_coalition_from_voting_behaviour_per_gp(
    legis_inits_with_votes: &[LegislativeInitiativeWithVotes],
) -> HashMap<String, Vec<String>> {
    let mut infavor_per_party_gp = HashMap::new();

    for legis_init in legis_inits_with_votes {
        for vote in legis_init.votes.as_ref().unwrap_or(&vec![]) {
            let key = (legis_init.gp.clone(), vote.party.clone());
            infavor_per_party_gp
                .entry(key)
                .and_modify(|x: &mut u32| *x += vote.infavor as u32)
                .or_default();
        }
    }

    let mut votes_with_infavor_count = HashMap::new();
    for ((gp, party), infavor_count) in &infavor_per_party_gp {
        votes_with_infavor_count
            .entry(gp)
            .and_modify(|x: &mut Vec<(&String, &u32)>| x.push((party, infavor_count)))
            .or_insert(vec![(party, infavor_count)]);
    }

    votes_with_infavor_count
        .into_iter()
        .map(|(gp, infavor_counts)| {
            let gp = gp.as_ref().expect("Only nullable because of view");
            let (_max_party, max_infavor_count) = infavor_counts
                .iter()
                .max_by(|(_party, infavor_count), (_b_party, b_infavor_count)| {
                    infavor_count.cmp(b_infavor_count)
                })
                .unwrap();
            if gp.as_str() == "XXII" {
                (
                    gp.to_string(),
                    vec!["ÖVP".to_string(), "F".to_string(), "F-BZÖ".to_string()],
                )
            } else {
                (
                    gp.to_string(),
                    infavor_counts
                        .iter()
                        .filter(|(_party, infavor_count)| {
                            (**infavor_count as f32 / **max_infavor_count as f32) > 0.9
                        })
                        .map(|(party, _)| party.to_string())
                        .collect(),
                )
            }
        })
        .collect::<HashMap<_, _>>()
}
