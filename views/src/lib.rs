use crate::{
    decrees::create_ministerial_decrees_with_docs_view,
    delegates::create_delegates_view,
    gov_proposals::create_gov_proposals_view,
    parliament_qa::create_parliament_qa_view,
    views::{speeches::create_speeches_view, volksbgs::create_volksbgs_view},
    vote_results::create_vote_results_view,
};
use sqlx::{Postgres, Transaction};

mod views;
pub use views::*;

pub mod composite_types;

pub use composite_types::create_composite_types;

pub async fn create_views<'a>(tx: &mut Transaction<'a, Postgres>, up: bool) -> sqlx::Result<()> {
    create_ministerial_decrees_with_docs_view(tx, up).await?;
    create_delegates_view(tx, up).await?;
    create_speeches_view(tx, up).await?;
    create_vote_results_view(tx, up).await?;
    create_gov_proposals_view(tx, up).await?;
    create_parliament_qa_view(tx, up).await?;
    create_volksbgs_view(tx, up).await?;

    Ok(())
}
