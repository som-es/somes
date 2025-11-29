use sqlx::{Postgres, Transaction};

pub async fn create_vote_results_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS vote_results;")
        .execute(&mut **tx)
        .await?;
    sqlx::query!(
        "
        CREATE VIEW vote_results AS
        SELECT
          /* scalar */
          li.id AS id,
          NULL::optional_meilisearch_helper as \"meilisearch_helper: OptionalMeilisearchHelper\",
          /* legislative initiative row */
          (
            SELECT
              ROW(
                id, ityp, doktyp, gp, inr, emphasis,
                title, description, accepted, created_at,
                appeared_at, updated_at, requires_simple_majority,
                pre_declined_type, voted_by_name,
                plenary_session_id, is_emphasis_ai_generated,
                is_law, law_accepted, law_come_into_effect_date,
                law_expires_on_date, by_publication,
                has_reference, is_voteable_on, is_urgent,
                voting
              )::optional_db_legislative_initiative_query
            FROM
              legislative_initiatives
            WHERE
              id = li.id
          ) AS \"legislative_initiative: OptionalDbLegislativeInitiativeQuery\",
          /* votes */
          (
            CASE WHEN EXISTS (
              SELECT 1 FROM named_vote_info nvi WHERE nvi.legis_init_id = li.id
            ) THEN ARRAY(
              SELECT
                ROW(
                  m.party, NULL, COUNT(*), nv.infavor
                )::optional_db_vote
              FROM
                named_vote_info nvi
                JOIN named_votes nv ON nvi.id = nv.named_vote_info_id
                JOIN mandates m ON m.delegate_id = nv.delegate_id
              WHERE
                nvi.legis_init_id = li.id
                AND m.name LIKE '%Abge%National%'
                AND m.start_date <= li.created_at
                AND (COALESCE(m.end_date, li.created_at) >= li.created_at)
                AND nv.infavor IS NOT NULL
                AND m.party IS NOT NULL
              GROUP BY
                m.party,
                nv.infavor
            ) ELSE ARRAY(
              SELECT
                ROW(party, NULL, fraction, infavor)::optional_db_vote
              FROM
                votes v
              WHERE
                v.legislative_initiatives_id = li.id
            ) END
          ) AS \"votes: Vec<OptionalDbVote>\",
          /* speeches */
          ARRAY(
            SELECT
              ROW(
                delegate_id,
                legis_init_id,
                CASE WHEN opinion = 'Pro' THEN (
                  li.pre_declined_type NOT LIKE '%p%'
                ) WHEN opinion = 'Contra' THEN (
                  li.pre_declined_type LIKE '%p%'
                ) ELSE NULL END,
                duration_in_seconds,
                opinion,
                document_url,
                about
              )::optional_db_speech_with_link
            FROM
              plenar_speeches ps
              JOIN plenar_speech_links psl ON psl.plenar_speech_id = ps.id
              JOIN plenar_speech_legis_inits pl ON pl.speech_id = ps.id
              JOIN debates deb ON deb.id = ps.debate_id
            WHERE
              pl.legis_init_id = li.id
          ) AS \"speeches: Vec<OptionalDbSpeechWithLink>\",
          /* named votes */
          (
            SELECT
              ROW(
                ROW(
                  nvi.pro_count, nvi.contra_count,
                  nvi.given_vote_sum, nvi.invalid_count
                )::optional_db_named_vote_info,
                ARRAY(
                  SELECT
                    ROW(
                      id, infavor, was_absent, lev, similiarity_score,
                      searched_with, matched_with,
                      delegate_id, manually_matched
                    )::optional_db_named_vote
                  FROM
                    named_votes nv
                  WHERE
                    nv.named_vote_info_id = nvi.id
                )
              )::optional_db_named_votes
            FROM
              named_vote_info nvi
            WHERE
              nvi.legis_init_id = li.id
              AND li.voted_by_name
            LIMIT
              1
          ) AS \"named_votes: OptionalDbNamedVotes\",
          /* topics */
          ARRAY(
            SELECT
              ROW(topic)::optional_topic
            FROM
              topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"topics: Vec<OptionalTopic>\",
          ARRAY(
            SELECT
              ROW(topic)::optional_topic
            FROM
              eurovoc_topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"eurovoc_topics: Vec<OptionalTopic>\",
          ARRAY(
            SELECT
              ROW(topic)::optional_topic
            FROM
              other_keyword_topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"other_keyword_topics: Vec<OptionalTopic>\",
          /* documents */
          ARRAY(
            SELECT
              ROW(
                title, document_url, document_type
              )::document
            FROM
              legislative_documents
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"documents: Vec<Document>\",
          /* absences */
          (
            SELECT
              ARRAY(
                SELECT
                  delegate_id
                FROM
                  absences a
                WHERE
                  a.plenary_session_id = li.plenary_session_id
              )
          ) AS \"absences: Vec<i32>\",
          /* issued_by_dels */
          ARRAY(
            SELECT
              ROW(delegate_text, delegate_id)::optional_db_related_delegate
            FROM
              legis_init_delegates lid
            WHERE
              lid.legis_init_id = li.id
          ) AS \"issued_by_dels: Vec<OptionalDbRelatedDelegate>\",
          /* referenced_by_others_ids */
          (
            SELECT
              ARRAY(
                SELECT
                  origin_legis_init_id
                FROM
                  legis_inits_refs lir
                  JOIN legislative_initiatives li2 ON li2.id = origin_legis_init_id
                WHERE
                  lir.ref_gp = li.gp
                  AND lir.ref_ityp = li.ityp
                  AND lir.ref_inr = li.inr
                  AND li2.is_voteable_on
              )
          ) AS \"referenced_by_others_ids: Vec<i32>\",
          /* references */
          (
            SELECT
              ARRAY(
                SELECT
                  ROW(ref_gp, ref_ityp, ref_inr)::optional_db_reference
                FROM
                  legis_inits_refs lir
                  JOIN legislative_initiatives li2 ON li2.gp = lir.ref_gp
                  AND li2.ityp = lir.ref_ityp
                  AND li2.inr = lir.ref_inr
                WHERE
                  origin_legis_init_id = li.id
                  AND li2.is_voteable_on
              )
          ) AS \"references: Vec<OptionalDbReference>\"
        FROM
          legislative_initiatives li
        "
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}
