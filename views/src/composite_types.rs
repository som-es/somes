use dataservice::combx::{
    DbAiSummary, DbLegislativeInitiativeQuery, DbMinistrialProposalQueryMeta, DbNamedVote,
    DbNamedVoteInfo, DbNamedVotes, DbPartyNamedVoteCount, DbReference, DbRelatedDelegate,
    DbSpeechWithLink, DbVote, MeilisearchHelper, Topic, VoteResult,
};
use somes_common_lib::{Document, FullMandate, ToCompositeType};
use sqlx::{Postgres, Transaction};

#[macro_export]
macro_rules! run_composite_type_creation {
    ($pool:ident, $($ty:ident),*) => {
        $(
            create_composite_type::<$ty>($pool).await?;
        )*
    };
}

pub async fn create_composite_types<'a>(pool: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    run_composite_type_creation!(
        pool,
        DbAiSummary,
        FullMandate,
        Document,
        DbRelatedDelegate,
        DbReference,
        DbPartyNamedVoteCount,
        DbNamedVote,
        DbNamedVoteInfo,
        DbNamedVotes,
        DbSpeechWithLink,
        DbVote,
        DbLegislativeInitiativeQuery,
        Topic,
        Topic,
        MeilisearchHelper,
        DbMinistrialProposalQueryMeta,
        VoteResult
    );
    Ok(())
}

pub async fn create_composite_type<'a, T: ToCompositeType>(
    tx: &mut Transaction<'a, Postgres>,
) -> sqlx::Result<()> {
    let create_composite_type_str = T::to_sql_create_composite_type();
    sqlx::query(&format!("DROP TYPE IF EXISTS {} cascade", T::type_name()))
        .execute(&mut **tx)
        .await?;
    println!("Creating composite type: {}", create_composite_type_str);
    sqlx::query(&create_composite_type_str)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
