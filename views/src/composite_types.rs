use combx::{
    DbAiInquiry, DbAiSummary, DbAnswerEntry, DbInterjection, DbLegislativeInitiativeQuery,
    DbMinistrialProposalQueryMeta, DbNamedVote, DbNamedVoteInfo, DbNamedVotes,
    DbPartyNamedVoteCount, DbReference, DbRelatedDelegate, DbSpeechAiSummary, DbSpeechRelations,
    DbSpeechWithLink, DbVolksbg, DbVote, DelegateMatch, FullMandate, FullSpeech, Interjection,
    MeilisearchHelper, ParliamentAnswer, ParliamentInquiry, ParliamentInquiryResponse,
    ParliamentRawData, Topic, VoteResult,
};
use somes_common_lib::{Document, ToCompositeType};
use sqlx::{Postgres, Transaction};

#[macro_export]
macro_rules! run_composite_type_creation {
    ($pool:ident, $up:ident, $($ty:ident),*) => {
        $(
            create_composite_type::<$ty>($pool, $up).await?;
        )*
    };
}

pub async fn create_composite_types<'a>(
    pool: &mut Transaction<'a, Postgres>,
    up: bool,
) -> sqlx::Result<()> {
    run_composite_type_creation!(
        pool,
        up,
        DbInterjection,
        DelegateMatch,
        Interjection,
        DbSpeechWithLink,
        DbSpeechAiSummary,
        DbSpeechRelations,
        FullSpeech,
        DbAnswerEntry,
        DbAiInquiry,
        DbAiSummary,
        FullMandate,
        Document,
        DbRelatedDelegate,
        DbReference,
        DbPartyNamedVoteCount,
        DbNamedVote,
        DbNamedVoteInfo,
        DbNamedVotes,
        DbVote,
        DbLegislativeInitiativeQuery,
        Topic,
        Topic,
        MeilisearchHelper,
        DbMinistrialProposalQueryMeta,
        VoteResult,
        ParliamentRawData,
        ParliamentInquiry,
        ParliamentAnswer,
        ParliamentInquiryResponse,
        DbVolksbg
    );
    Ok(())
}

pub async fn create_composite_type<'a, T: ToCompositeType>(
    tx: &mut Transaction<'a, Postgres>,
    up: bool,
) -> sqlx::Result<()> {
    sqlx::query(&format!("DROP TYPE IF EXISTS {} cascade", T::type_name()))
        .execute(&mut **tx)
        .await?;
    if up {
        let create_composite_type_str = T::to_sql_create_composite_type();
        println!("Creating composite type: {}", create_composite_type_str);
        sqlx::query(&create_composite_type_str)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}
