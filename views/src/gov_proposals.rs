use dataservice::combx::{DbAiSummary, DbMinistrialProposalQueryMeta};
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_gov_proposals_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS gov_proposals;")
        .execute(&mut **tx)
        .await?;

    let ministrial_proposal_fields = DbMinistrialProposalQueryMeta::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "inner_mp.id" } else { field })
        .collect::<Vec<_>>()
        .join(" ,");

    let summary_fields = DbAiSummary::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "s.id" } else { field })
        .collect::<Vec<_>>()
        .join(" ,");

    sqlx::query(&format!(
        "
    CREATE VIEW gov_proposals AS
    SELECT
        mp.id,
        (SELECT ROW(
          {ministrial_proposal_fields}
)::db_ministrial_proposal_query_meta
        from 
            ministrial_proposals inner_mp 
        where
            inner_mp.id = mp.id
        ) as \"ministrial_proposal: DbMinistrialProposalQueryMeta\",
        (select ROW(
                vr.*
            )::vote_result
            from vote_results vr 
            where vr.id = 
            (select li.id 
            from 
                legislative_initiatives li
            where gp = mp.legis_init_gp 
            and inr = mp.legis_init_inr 
            and ityp = mp.legis_init_ityp)::int
        ) as \"vote_result: OptionalVoteResult\",
        ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              topics_ministrial_proposals
            WHERE
              ministrial_proposal_id = mp.id
          ) AS \"topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              eurovoc_topics_ministrial_proposals
            WHERE
              ministrial_proposal_id = mp.id
          ) AS \"eurovoc_topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              other_keyword_topics_ministrial_proposals
            WHERE
              ministrial_proposal_id = mp.id
          ) AS \"other_keyword_topics: Vec<Topic>\",
          /* documents */
          ARRAY(
            SELECT
              ROW(
                title, document_url, document_type
              )::document
            FROM
              ministrial_proposals_documents
            WHERE
              ministrial_proposal_id = mp.id
          ) AS \"documents: Vec<Document>\",
            /* ministerial_issuers */
        ( SELECT
           ARRAY(
            SELECT
                delegate_id 
            FROM
              ministrial_issuer
            WHERE
              ministrial_proposal_id = mp.id
           ) 
        ) AS \"ministerial_issuers: Vec<i32>\",
        (
            SELECT
              ROW(
                {summary_fields}
              )::db_ai_summary
            FROM
              ministerial_proposal_summaries mps
              inner join summaries s on s.id = mps.summary_id
            WHERE
              mps.ministerial_proposal_id = mp.id
            order by
              s.generated_at DESC
            LIMIT 1
          ) AS \"ai_summary: DbAiSummary\"

        from ministrial_proposals mp
        "
    ))
    .execute(&mut **tx)
    .await?;
    Ok(())
}
