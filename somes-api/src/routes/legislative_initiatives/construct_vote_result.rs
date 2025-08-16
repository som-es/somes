use dataservice::{
    combx::VoteResult,
    db::models::DbLegislativeInitiativeQuery,
};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::get_json_cache;

const JSON_VOTE_RESULT_SQL: &str = include_str!("./json_vote_result.sql");

pub async fn construct_vote_result(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    legis_init: DbLegislativeInitiativeQuery,
) -> sqlx::Result<VoteResult> {
    let key = format!("vote_result/{}", legis_init.id.to_string());
    let res = get_json_cache::<VoteResult>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let start = std::time::Instant::now();

    /*
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

    log::info!("votes: {votes:?}");

    let out = VoteResult {
        id: legis_init.id,
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
    };*/

    let out = sqlx::query!(
        "
    WITH DelegateParty AS (
    SELECT 
        delegates.id AS id, 
        mandates.party AS party 
    FROM 
        mandates 
    INNER JOIN 
        delegates ON delegates.id = mandates.delegate_id 
    WHERE 
        mandates.name LIKE '%Abge%National%' 
        AND start_date <= $2 
        AND (CASE WHEN end_date IS NULL THEN $2 ELSE end_date END) >= $2 
),
PartyVoteCounts AS (
    SELECT 
        DelegateParty.party, 
        named_votes.infavor, 
        COUNT(*) AS count
    FROM 
        named_vote_info 
    INNER JOIN 
        named_votes ON named_vote_info.id = named_votes.named_vote_info_id 
    INNER JOIN 
        DelegateParty ON DelegateParty.id = named_votes.delegate_id
    WHERE 
        named_vote_info.legis_init_id = $1
        AND (named_votes.infavor = true OR named_votes.infavor = false)
        AND DelegateParty.party IS NOT NULL
    GROUP BY 
        DelegateParty.party, 
        named_votes.infavor
),
HasNamedVotes AS (
    SELECT EXISTS (
        SELECT 1 
        FROM named_vote_info 
        WHERE legis_init_id = $1
    ) AS has_named_votes
)
SELECT jsonb_build_object(
    'id', $1,
    'votes', (
        SELECT 
            COALESCE(
                CASE 
                    WHEN (SELECT has_named_votes FROM HasNamedVotes) THEN (
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'party', party,
                                'infavor', infavor,
                                'fraction', count,
                                'legislative_initiatives_id', $1 
                            )
                        )
                        FROM PartyVoteCounts
                    )
                    ELSE (
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'party', party,
                                'fraction', fraction,
                                'infavor', infavor,
                                'legislative_initiatives_id', legislative_initiatives_id
                            )
                        )
                        FROM votes
                        WHERE legislative_initiatives_id = $1
                    )
                END,
                '[]'::jsonb
            )
    ),
    'issued_by_dels', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'delegate_id', delegate_id,
                    'text', delegate_text
                )
            ),
            '[]'::jsonb
        )
        FROM legis_init_delegates
        WHERE legis_init_id = $1
    ),
    'named_votes', (
        SELECT jsonb_build_object(
            'named_vote_info', jsonb_build_object(
                'id', named_vote_info.id,
                'legis_init_id', named_vote_info.legis_init_id,
                'pro_count', named_vote_info.pro_count,
                'contra_count', named_vote_info.contra_count,
                'given_vote_sum', named_vote_info.given_vote_sum,
                'invalid_count', named_vote_info.invalid_count
            ),
            'named_votes', (
                SELECT COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'id', id,
                            'infavor', infavor,
                            'was_absent', was_absent,
                            'lev', lev,
                            'similiarity_score', similiarity_score,
                            'searched_with', searched_with,
                            'matched_with', matched_with,
                            'delegate_id', delegate_id,
                            'named_vote_info_id', named_vote_info_id,
                            'manually_matched', manually_matched
                        )
                    ),
                    '[]'::jsonb
                )
                FROM named_votes
                WHERE named_vote_info_id = named_vote_info.id
            )
        )
        FROM named_vote_info
        WHERE legis_init_id = $1
        AND $3
        LIMIT 1
    ),
    'speeches', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'about', about,
                    'delegate_id', delegate_id,
                    'infavor', CASE
                        WHEN opinion = 'Pro' THEN ($4 NOT LIKE '%p%')
                        WHEN opinion = 'Contra' THEN ($4 LIKE '%p%')
                        ELSE NULL
                    END,
                    'opinion', opinion,
                    'document_url', document_url,
                    'duration_in_seconds', duration_in_seconds,
                    'legislative_initiatives_id', $1
                )
            ),
            '[]'::jsonb
        )
        FROM plenar_speeches
        INNER JOIN plenar_speech_links psl ON plenar_speeches.id = psl.plenar_speech_id
        inner join plenar_speech_legis_inits pl on pl.speech_id = plenar_speeches.id
        inner join debates on debates.id = plenar_speeches.debate_id
        WHERE legis_init_id = $1
    ),
    'topics', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'topic', topic
                )
            ),
            '[]'::jsonb
        )
        FROM topics_legis_init
        WHERE legislative_initiatives_id = $1
    ),
    'eurovoc_topics', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'topic', topic
                )
            ),
            '[]'::jsonb
        )
        FROM eurovoc_topics_legis_init
        WHERE legislative_initiatives_id = $1
    ),
    'other_keyword_topics', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'topic', topic
                )
            ),
            '[]'::jsonb
        )
        FROM other_keyword_topics_legis_init
        WHERE legislative_initiatives_id = $1
    ),
    'documents', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'title', title,
                    'document_url', document_url,
                    'document_type', document_type
                )
            ),
            '[]'::jsonb
        )
        FROM legislative_documents
        WHERE legislative_initiatives_id = $1
    ),
   'absences', (
        SELECT COALESCE(
            jsonb_agg(delegate_id), 
            '[]'::jsonb
        )
        FROM absences
        WHERE plenary_session_id = (
            SELECT plenary_session_id
            FROM legislative_initiatives
            WHERE id = $1
        )
    ), 
    'referenced_by_others_ids', (
        SELECT COALESCE(
            jsonb_agg(origin_legis_init_id), 
            '[]'::jsonb
        )
        FROM legis_inits_refs
        WHERE ref_gp = $5 and ref_ityp = $6 and ref_inr = $7
    ), 
    'references', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'gp', ref_gp,
                    'ityp', ref_ityp,
                    'inr', ref_inr
                )
            ), 
            '[]'::jsonb
        )
        FROM legis_inits_refs
        --inner join legislative_initiatives li on li.gp = ref_gp and li.ityp = ref_ityp and li.inr = ref_inr
        WHERE origin_legis_init_id = $1
    ), 
    'legislative_initiative', (
        SELECT row_to_json(legislative_initiatives)
        FROM legislative_initiatives
        WHERE id = $1
    )
) AS result_json;

    ",
        legis_init.id,
        legis_init.created_at,
        legis_init.voted_by_name,
        legis_init.pre_declined_type,
        legis_init.gp,
        legis_init.ityp,
        legis_init.inr,
    )
    .fetch_one(pg)
    .await
    .map(|x| {
        // let out = serde_json::from_value::<VoteResult>(x.result_json.clone().unwrap());
        // if out.is_err() {
        //     log::info!("{}", x.result_json.as_ref().unwrap());

        // }
        match serde_json::from_value::<VoteResult>(x.result_json.unwrap()) {
            Err(e) => {
                panic!("could not parse vote result from value: {e:?}, legis init: {legis_init:?}")
            }
            Ok(val) => val,
        }
    })?;

    // log::info!("gp: {}", out.legislative_initiative.gp);
    crate::set_json_cache_with_relevance(
        &mut redis_con,
        &key,
        &out,
        out.legislative_initiative.created_at,
    )
    .await
    .ok_or(sqlx::Error::WorkerCrashed)?;
    // log::info!("elapsed: {:?}", start.elapsed());
    // Ok(out)
    Ok(out)
}
