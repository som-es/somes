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
        AND start_date <= CURRENT_DATE
        AND (CASE WHEN end_date IS NULL THEN CURRENT_DATE ELSE end_date END) >= CURRENT_DATE
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
        LIMIT 1
    ),
    
    'speeches', (
        SELECT COALESCE(
            jsonb_agg(
                jsonb_build_object(
                    'delegate_id', delegate_id,
                    'infavor', infavor,
                    'opinion', opinion,
                    'document_url', document_url
                )
            ),
            '[]'::jsonb
        )
        FROM speeches
        INNER JOIN speeches_html_urls ON speeches.id = speeches_html_urls.speech_id
        WHERE legislative_initiatives_id = $1
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
        FROM eurovoc_topics_legis_init
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
    'legislative_initiative', (
        SELECT row_to_json(legislative_initiatives)
        FROM legislative_initiatives
        WHERE id = $1
    )
) AS result_json;
