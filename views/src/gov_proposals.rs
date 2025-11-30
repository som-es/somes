use sqlx::{Postgres, Transaction};

pub async fn create_gov_proposals_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS gov_proposals;")
        .execute(&mut **tx)
        .await?;

    sqlx::query!(
        "
    CREATE VIEW gov_proposals AS
    SELECT
        mp.id,
        (SELECT ROW(
            inner_mp.id, 
            ityp, 
            gp, 
            inr, 
            emphasis, 
            title, 
            description, 
            created_at, 
            updated_at, 
            due_to, 
            ressort, 
            ressort_shortform, 
            legis_init_gp, 
            legis_init_inr, 
            legis_init_ityp,
            has_vote_result)::db_ministrial_proposal_query_meta
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
        ) as \"vote_result: VoteResult\",
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
        ) AS \"ministerial_issuers: Vec<i32>\"

        from ministrial_proposals mp
        "
    ).execute(&mut **tx).await?;
    Ok(())
}
