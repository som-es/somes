mod all_props;
mod by_path;
mod construct_gov_proposal;
mod db;
mod gov_props_by_search;
pub use all_props::*;
pub use by_path::*;
pub use construct_gov_proposal::*;
pub use db::*;
pub use gov_props_by_search::*;

use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use dataservice::db::models::DbMinistrialProposalQueryMeta;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::{GovPropFilter, Page, LATEST, LIVE, SEARCH};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;

use crate::{
    routes::latest_gov_proposals, server::AppState, PgPoolConnection, RedisConnection,
    GOV_PROPS_PER_PAGE,
};

use super::{
    delegate_by_id_sqlx,
    statistics::filtering::{bind_values, build_filter, count_filter, IntoFilterArgument, Manual},
    GovProposalDelegate, LegisInitErrorResponse,
};

pub fn create_gov_proposals_router() -> Router<AppState> {
    Router::new()
        .route(SEARCH, post(gov_props_by_search))
        .route(LIVE, post(gov_proposals_per_page))
        .route(LATEST, post(latest_gov_proposals))
        .route("/{gp}/{inr}", get(gov_proposal_by_path))
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct GovProposalsWithMaxPage {
    pub gov_proposals: Vec<GovProposalDelegate>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn construct_gov_delegate_proposal(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    ministrial_proposal: DbMinistrialProposalQueryMeta,
) -> sqlx::Result<GovProposalDelegate> {
    let gov_proposal = construct_gov_proposal(redis_con.clone(), &pg, ministrial_proposal)
        .await
        .unwrap();
    // TODO: display multiple gov officials if there are multiple ministerial issuers
    let mut delegates = vec![];
    for ministerial_issuer in gov_proposal.ministerial_issuers.as_deref().unwrap_or(&[]) {
        let delegate = delegate_by_id_sqlx(*ministerial_issuer, &pg, redis_con.clone()).await?;
        delegates.push(delegate);
    }
    Ok(GovProposalDelegate {
        gov_proposal,
        delegate: delegates.into_iter().next(), // TODO: handle multiple delegates properly
    })
}

pub async fn gov_proposals_per_page(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(gov_prop_filter): Json<Option<GovPropFilter>>,
) -> Result<Json<GovProposalsWithMaxPage>, LegisInitErrorResponse> {
    if page.page < 0 {
        return Err(LegisInitErrorResponse::InvalidPage);
    }
    let (ministrial_proposals, entry_count) = filtered_ministrial_proposals(
        &pg,
        page.page,
        GOV_PROPS_PER_PAGE.parse().unwrap_or(12),
        gov_prop_filter,
    )
    .await
    .map_err(|e| {
        LegisInitErrorResponse::GenericErrorResponse(crate::GenericErrorResponse::DbSelectFailure(
            Some(e),
        ))
    })?;

    futures::future::join_all(
        ministrial_proposals
            .into_iter()
            .map(|ministrial_proposal| {
                construct_gov_delegate_proposal(redis_con.clone(), &pg, ministrial_proposal)
            })
            .into_iter(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<_>>>()
    .map(|gov_proposals| GovProposalsWithMaxPage {
        gov_proposals,
        entry_count,
        max_page: (entry_count as f64 / GOV_PROPS_PER_PAGE.parse().unwrap_or(12.)).ceil() as i64,
    })
    .map(Json)
    .map_err(|e| {
        LegisInitErrorResponse::GenericErrorResponse(crate::GenericErrorResponse::DbSelectFailure(
            Some(e),
        ))
    })
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Copy, FromRow)]
pub struct Count {
    count: i64,
}

pub async fn filtered_ministrial_proposals(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    ministrial_prop_filter: Option<GovPropFilter>,
) -> sqlx::Result<(Vec<DbMinistrialProposalQueryMeta>, i64)> {
    let ministrial_prop_filter = ministrial_prop_filter.unwrap_or_default();

    let filter_arg = ministrial_prop_filter.legis_period.with_sql_column("gp");
    let filter_arg1 = ministrial_prop_filter
        .has_vote_result
        .with_sql_column("has_vote_result");
    let filter_arg2 = Manual("true").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let filter = build_filter(&filters);
    let filter_count = count_filter(&filters);

    let query = format!(
        "select
            mi.delegate_id,
            mp.id,
            mp.ityp,
            mp.gp,
            mp.inr,
            mp.emphasis,
            mp.title,
            mp.description,
            mp.created_at,
            mp.updated_at,
            mp.due_to,
            mp.ressort,
            mp.ressort_shortform,
            mp.legis_init_gp,
            mp.legis_init_inr,
            mp.legis_init_ityp,
            mp.has_vote_result
        from
            ministrial_issuer as mi
        inner join
            ministrial_proposals as mp
        on
            mp.id = mi.ministrial_proposal_id
        where
            {filter}
        order by created_at DESC
        offset ${} limit ${};",
        filter_count,
        filter_count + 1
    );

    let mut filtered_query = sqlx::query_as::<_, DbMinistrialProposalQueryMeta>(&query);
    filtered_query = bind_values(filtered_query, &filters);
    filtered_query = filtered_query.bind(page * page_elements);
    filtered_query = filtered_query.bind(page_elements);

    let query = format!(
        "select
        COUNT(*)
    from
        ministrial_issuer as mi
    inner join
        ministrial_proposals as mp
    on
        mp.id = mi.ministrial_proposal_id
    where
        {filter}"
    );

    let mut count_filter_query = sqlx::query_as::<_, Count>(&query);

    let filter_arg = ministrial_prop_filter.legis_period.with_sql_column("gp");
    let filter_arg1 = ministrial_prop_filter
        .has_vote_result
        .with_sql_column("has_vote_result");

    let filter_arg2 = Manual("true").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];
    count_filter_query = bind_values(count_filter_query, &filters);

    Ok((
        filtered_query.fetch_all(pg).await?,
        count_filter_query.fetch_one(pg).await?.count,
    ))
}

pub async fn get_latest_ministrial_proposals_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<GovPropFilter>,
) -> sqlx::Result<(Vec<DbMinistrialProposalQueryMeta>, i64)> {
    filtered_ministrial_proposals(pg, page, page_elements, filter).await
}

#[cfg(test)]
mod tests {
    use dataservice::connect_pg;
    use somes_common_lib::GovPropFilter;

    use crate::routes::get_latest_ministrial_proposals_per_page;

    #[tokio::test]
    async fn test_filtered_ministrial_prop() {
        let pg = connect_pg().await;
        let entries = get_latest_ministrial_proposals_per_page(&pg, 1, 10, None)
            .await
            .unwrap();
        println!("entries: {entries:?}");

        let entries = get_latest_ministrial_proposals_per_page(
            &pg,
            1,
            10,
            Some(GovPropFilter {
                has_vote_result: Some(true),
                legis_period: None,
            }),
        )
        .await
        .unwrap();
        println!("entries: {entries:?}");
    }
}
