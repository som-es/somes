use dataservice::combx::{DbAiSummary, DbLegislativeInitiativeQuery};
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_vote_results_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS vote_results;")
        .execute(&mut **tx)
        .await?;

    let legis_init_fields = DbLegislativeInitiativeQuery::field_orders().join(" ,");
    let summary_fields = DbAiSummary::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "s.id" } else { field })
        .collect::<Vec<_>>()
        .join(" ,");

    sqlx::query(
        &format!("
        CREATE VIEW vote_results AS
        SELECT
          /* scalar */
          li.id AS id,
          /* legislative initiative row */
          (
            SELECT
              ROW(
                {legis_init_fields}
              )::db_legislative_initiative_query
            FROM
              legislative_initiatives
            WHERE
              id = li.id
          ) AS \"legislative_initiative: DbLegislativeInitiativeQuery\",
          /* votes */
          (
            CASE WHEN EXISTS (
              SELECT 1 FROM named_vote_info nvi WHERE nvi.legis_init_id = li.id
            ) THEN ARRAY(
              SELECT
                ROW(
                  m.party, NULL, COUNT(*), nv.infavor
                )::db_vote
              FROM
                named_vote_info nvi
                JOIN named_votes nv ON nvi.id = nv.named_vote_info_id
                JOIN mandates m ON m.delegate_id = nv.delegate_id
              WHERE
                nvi.legis_init_id = li.id
                AND m.name LIKE '%Abge%National%'
                AND m.start_date <= li.nr_plenary_activity_date
                AND (COALESCE(m.end_date, li.nr_plenary_activity_date) >= li.nr_plenary_activity_date)
                AND nv.infavor IS NOT NULL
                AND m.party IS NOT NULL
              GROUP BY
                m.party,
                nv.infavor
            ) ELSE ARRAY(
              SELECT
                ROW(party, NULL, fraction, infavor)::db_vote
              FROM
                votes v
              WHERE
                v.legislative_initiatives_id = li.id
            ) END
          ) AS \"votes: Vec<DbVote>\",
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
              )::db_speech_with_link
            FROM
              plenar_speeches ps
              JOIN plenar_speech_links psl ON psl.plenar_speech_id = ps.id
              JOIN plenar_speech_legis_inits pl ON pl.speech_id = ps.id
              JOIN debates deb ON deb.id = ps.debate_id
            WHERE
              pl.legis_init_id = li.id
          ) AS \"speeches: Vec<DbSpeechWithLink>\",
          /* named votes */
          (
            SELECT
              ROW(
                ROW(
                  nvi.pro_count, nvi.contra_count,
                  nvi.given_vote_sum, nvi.invalid_count
                )::db_named_vote_info,
                ARRAY(
                  SELECT
                    ROW(
                      id, infavor, was_absent, lev, similiarity_score,
                      searched_with, matched_with,
                      delegate_id, manually_matched
                    )::db_named_vote
                  FROM
                    named_votes nv
                  WHERE
                    nv.named_vote_info_id = nvi.id
                )
              )::db_named_votes
            FROM
              named_vote_info nvi
            WHERE
              nvi.legis_init_id = li.id
              AND li.voted_by_name
            LIMIT
              1
          ) AS \"named_votes: DbNamedVotes\",
          /* topics */
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              eurovoc_topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"eurovoc_topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              other_keyword_topics_legis_init
            WHERE
              legislative_initiatives_id = li.id
          ) AS \"other_keyword_topics: Vec<Topic>\",
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
              ROW(delegate_text, delegate_id)::db_related_delegate
            FROM
              legis_init_delegates lid
            WHERE
              lid.legis_init_id = li.id
          ) AS \"issued_by_dels: Vec<DbRelatedDelegate>\",
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
                  ROW(ref_gp, ref_ityp, ref_inr)::db_reference
                FROM
                  legis_inits_refs lir
                  JOIN legislative_initiatives li2 ON li2.gp = lir.ref_gp
                  AND li2.ityp = lir.ref_ityp
                  AND li2.inr = lir.ref_inr
                WHERE
                  origin_legis_init_id = li.id
                  AND li2.is_voteable_on
              )
          ) AS \"references: Vec<DbReference>\",
          (
            SELECT
              ROW(
                {summary_fields}
              )::db_ai_summary
            FROM
              legislative_initiative_summaries lis
              inner join summaries s on s.id = lis.summary_id
            WHERE
              lis.legis_init_id = li.id
            order by
              s.generated_at DESC
            LIMIT 1
          ) AS \"ai_summary: DbAiSummary\",
          (
          SELECT
            ROW(
              /* votes */
              ARRAY[]::text[],

              /* issuer_parties */
              ARRAY(
                SELECT
                  DISTINCT m.party
                FROM
                  legis_init_delegates lid
                  JOIN mandates m
                    ON m.delegate_id = lid.delegate_id
                WHERE
                  lid.legis_init_id = li.id
                  AND m.is_nr
                  AND m.start_date <= li.nr_plenary_activity_date::date
                  AND COALESCE(m.end_date, li.nr_plenary_activity_date::date) >= li.nr_plenary_activity_date::date
                  AND m.party IS NOT NULL
              )
            )::meilisearch_helper
        ) AS \"meilisearch_helper: MeilisearchHelper\"
        FROM
          legislative_initiatives li
        "
        ))
    .execute(&mut **tx)
    .await?;

    sqlx::query!("DROP materialized VIEW IF EXISTS latest_legislative_initiatives;")
        .execute(&mut **tx)
        .await?;

    sqlx::query!("
        create materialized view latest_legislative_initiatives as
        select * from legislative_initiatives
            where nr_plenary_activity_date = (select MAX(nr_plenary_activity_date) from legislative_initiatives
            where accepted is not null) and accepted is not null and is_voteable_on
    ").execute(&mut **tx)
    .await?;

    if std::env::var("FULL_VIEW_UPDATE")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or_default()
    {
        sqlx::query!("DROP materialized VIEW IF EXISTS legislative_initiatives_with_votes;")
            .execute(&mut **tx)
            .await?;

        sqlx::query!(
            r#"
        create materialized view legislative_initiatives_with_votes as
        SELECT
            li.id,
            li.gp,
            ARRAY(
                SELECT ROW(v.party, NULL, v.fraction, v.infavor)::db_vote
                FROM votes v
                WHERE v.legislative_initiatives_id = li.id
            ) AS "votes: Vec<DbVote>"
        FROM
            legislative_initiatives li
        WHERE
            accepted = 'a'
    "#
        )
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
