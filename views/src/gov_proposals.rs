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
            delegate_id,
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
            ministrial_issuer as mi 
        inner join 
            ministrial_proposals inner_mp 
        on 
            inner_mp.id = mi.ministrial_proposal_id
        where
            inner_mp.id = mp.id
        ) as \"ministrial_proposal: DbMinistrialProposalQueryMeta\",
        (select ROW(
                vr.\"id\",
                vr.\"legislative_initiative: DbLegislativeInitiativeQuery\",
                vr.\"votes: Vec<DbVote>\",
                vr.\"speeches: Vec<DbSpeechWithLink>\",
                vr.\"named_votes: DbNamedVotes\",
                vr.\"topics: Vec<Topic>\",
                vr.\"eurovoc_topics: Vec<Topic>\",
                vr.\"other_keyword_topics: Vec<Topic>\",
                vr.\"documents: Vec<Document>\",
                vr.\"absences: Vec<i32>\",
                vr.\"issued_by_dels: Vec<DbRelatedDelegate>\",
                vr.\"referenced_by_others_ids: Vec<i32>\",
                vr.\"references: Vec<DbReference>\",
                vr.\"meilisearch_helper: MeilisearchHelper\"
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
              topics_ministrial_proposal
            WHERE
              ministerial_proposal_id = mp.id
          ) AS \"topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              eurovoc_topics_ministrial_proposal
            WHERE
              ministerial_proposal_id = mp.id
          ) AS \"eurovoc_topics: Vec<Topic>\",
          ARRAY(
            SELECT
              ROW(topic)::topic
            FROM
              other_keyword_topics_ministrial_proposal
            WHERE
              ministerial_proposal_id = mp.id
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
              ministerial_proposal_id = mp.id
          ) AS \"documents: Vec<Document>\",
        ARRAY(
            SELECT
              ROW(
                delegate_id 
              )::int
            FROM
              ministrial_issuer
            WHERE
              ministerial_proposal_id = mp.id
          ) AS \"ministerial_issuers: Vec<i32>\"

        from ministrial_proposals mp
        "
    ).execute(&mut **tx).await?;
    Ok(())
}
