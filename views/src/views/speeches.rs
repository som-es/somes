use combx::{DbSpeechAiSummary, DbSpeechRelations};
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_speeches_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS full_speeches;")
        .execute(&mut **tx)
        .await?;

    let summary_fields = DbSpeechAiSummary::field_orders()
        .into_iter()
        .map(|field| format!("sai.{field}"))
        .collect::<Vec<_>>()
        .join(" ,");
    let relation_fields = DbSpeechRelations::field_orders()
        .into_iter()
        .map(|field| format!("sr.{field}"))
        .collect::<Vec<_>>()
        .join(" ,");

    sqlx::query(&format!(
        "
        CREATE VIEW full_speeches AS
        SELECT
          /* scalar */
          ps_top.id AS id,
          ps_top.debate_id,
          ps_top.delegate_id,
          (
          SELECT
            ROW(
                delegate_id,
                array_remove(ARRAY(
                    SELECT legis_init_id
                    FROM plenar_speech_legis_inits
                    WHERE speech_id = ps.id
                ), NULL),
                CASE WHEN opinion = 'Pro' THEN (
                    true
                ) WHEN opinion = 'Contra' THEN (
                    false
                ) ELSE NULL END,
                duration_in_seconds,
                opinion,
                ARRAY(
                    SELECT document_url
                    FROM plenar_speech_links
                    WHERE plenar_speech_id = ps.id
                ),
                about,
                start
            )::db_speech_with_link
            FROM
            plenar_speeches ps
            JOIN debates deb ON deb.id = ps.debate_id
            JOIN plenar_infos pi ON pi.id = deb.plenar_id
            WHERE
                ps.id = ps_top.id
          ) AS \"speech: DbSpeechWithLink\",
          (
            SELECT
            ROW(
                {summary_fields}
            )::db_speech_ai_summary
            FROM
                speech_ai_summaries sai
            WHERE
                sai.speech_id = ps_top.id
            order by
                sai.generated_at DESC
            LIMIT 1
        ) AS \"ai_summary: DbSpeechAiSummary\",
        ARRAY(
            SELECT DISTINCT ON (sr.legis_init_id)
                ROW({relation_fields})::db_speech_relations
            FROM speech_proposal_relations sr
            JOIN speech_ai_summaries sai ON sai.id = sr.speech_ai_summary_id
            WHERE sai.speech_id = ps_top.id
            ORDER BY sr.legis_init_id, sr.generated_at DESC
        ) AS \"relations: Vec<DbSpeechRelations>\"
        FROM
            plenar_speeches ps_top
    ",
    ))
    .execute(&mut **tx)
    .await?;

    Ok(())
}
