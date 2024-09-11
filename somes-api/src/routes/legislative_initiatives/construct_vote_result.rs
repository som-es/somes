use dataservice::{db::models::{DbLegislativeInitiativeQuery, DbVote}, with_data::named_votes};
use sqlx::PgPool;

use super::{get_absences_delegate_ids_sqlx, get_eurovoc_topics_from_legis_init, get_legis_docs_from_legis_init_sqlx, get_named_votes_from_legis_init_sqlx, get_speeches_from_legis_init_sqlx, get_votes_from_legis_init_sqlx, VoteResult};

pub async fn construct_vote_result(
    pg: &PgPool,
    legis_init: DbLegislativeInitiativeQuery,
) -> sqlx::Result<VoteResult> {
    let named_votes =
        get_named_votes_from_legis_init_sqlx(pg, legis_init.id, legis_init.voted_by_name).await?;
    let votes = if named_votes.is_some() {
        dataservice::with_data::named_votes::named_vote_pro_count_by_party(
            pg,
            legis_init.id,
            legis_init.created_at,
        )
        .await?
        .into_iter()
        .map(|vote| DbVote {
            party: vote.party.unwrap(),
            fraction: vote.count.unwrap() as i32,
            infavor: vote.infavor.unwrap(),
            legislative_initiatives_id: legis_init.id,
        })
        .collect()
    } else {
        get_votes_from_legis_init_sqlx(pg, legis_init.id).await?
    };

    Ok(VoteResult {
        votes,
        named_votes,
        speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
        topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
        documents: get_legis_docs_from_legis_init_sqlx(pg, legis_init.id).await?,
        absences: match legis_init.plenary_session_id {
            Some(plenary_session_id) => {
                get_absences_delegate_ids_sqlx(pg, plenary_session_id).await?
            }
            None => vec![],
        },
        legislative_initiative: legis_init,
    })
}
