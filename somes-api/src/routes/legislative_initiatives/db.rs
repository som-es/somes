use dataservice::db::{
    models::{DbLegislativeInitiativeQuery, DbSpeech, DbVote},
    schema::{
        legislative_initiatives::{accepted, created_at, dsl::legislative_initiatives},
        votes::{dsl::votes, legislative_initiatives_id},
    },
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use somes_common_lib::{DateRange, LegisInitFilter};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{dataservice::get_speeches_from_legis_init, today};

use super::LegisInitErrorResponse;

pub fn get_legislative_initiatives(
    con: &mut PgConnection,
    filter: DateRange,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        .filter(created_at.gt(filter.start))
        .filter(created_at.lt(filter.end))
        .filter(accepted.is_not_null())
        .load::<DbLegislativeInitiativeQuery>(con)
}

async fn filtered_legislative_initiatives(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: &LegisInitFilter,
) -> Result<(Vec<DbLegislativeInitiativeQuery>, i64), sqlx::Error> {

    let mut query = String::from(
        "SELECT DISTINCT * FROM legislative_initiatives WHERE ",
    );

    let mut param_index = 1;
    if filter.accepted.is_some() {
        query.push_str(&format!(" AND accepted = ${}", param_index));
        param_index += 1;
    }

    if filter.simple_majority.is_some() {
        query.push_str(&format!(" AND requires_simple_majority = ${}", param_index));
        param_index += 1;
    }

    if filter.legis_period.is_some() {
        query.push_str(&format!(" AND gp = ${}", param_index));
        param_index += 1;
    }

    let count_query = query.clone().replace('*', "COUNT(*)");

    query.push_str(&format!(
        " ORDER BY created_at DESC OFFSET ${} LIMIT ${}",
        param_index,
        param_index + 1
    ));

    let mut filtered_query = sqlx::query_as::<_, DbLegislativeInitiativeQuery>(&query);
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_query);

    if let Some(accepted_value) = filter.accepted {
        filtered_query = filtered_query.bind(accepted_value);
        count_query = count_query.bind(accepted_value);
    }
    if let Some(simple_majority) = filter.simple_majority {
        filtered_query = filtered_query.bind(simple_majority);
        count_query = count_query.bind(simple_majority);
    }
    if let Some(legis_period) = &filter.legis_period {
        filtered_query = filtered_query.bind(legis_period);
        count_query = count_query.bind(legis_period);
    }
    filtered_query = filtered_query.bind(page * page_elements).bind(page_elements);
    Ok((
        filtered_query.fetch_all(pg).await?,
        count_query.fetch_one(pg).await?.0,
    ))
}

pub async fn get_latest_legis_inits_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&LegisInitFilter>,
) -> sqlx::Result<(Vec<DbLegislativeInitiativeQuery>, i64)> {
    let res = match filter {
        Some(filter) => {
            filtered_legislative_initiatives(pg, page, page_elements, filter).await?
        }
        None => {
            (sqlx::query_as!(DbLegislativeInitiativeQuery,
                "select distinct * from legislative_initiatives where accepted is not null order by created_at desc offset $1 limit $2",
                page * page_elements,
                page_elements
            )
            .fetch_all(pg)
            .await?,

            sqlx::query!(
                "select distinct COUNT(*) from legislative_initiatives where accepted is not null",
            ).fetch_one(pg).await?.count.unwrap_or_default()
            )
        },
    };
    Ok(res)
}

pub async fn get_latest_legislative_initiatives_sqlx(
    pg: &PgPool,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query_as!(DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives 
            where created_at = (select MAX(created_at) from legislative_initiatives 
            where accepted is not null) and accepted is not null"
    )
    .fetch_all(pg)
    .await?;
    Ok(res)
}

pub fn get_latest_legislative_initiatives(
    con: &mut PgConnection,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        // make the date range configurable as user !
        .filter(created_at.gt(today() - chrono::Duration::days(90)))
        .filter(created_at.lt(today()))
        .filter(accepted.is_not_null())
        .order(created_at.desc())
        .load::<DbLegislativeInitiativeQuery>(con)
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct Topic {
    pub topic: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResult {
    pub legislative_initiative: DbLegislativeInitiativeQuery,
    pub votes: Vec<DbVote>,
    pub speeches: Vec<DbSpeech>,
    pub topics: Vec<Topic>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResultsWithMaxPage {
    pub vote_results: Vec<VoteResult>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn get_latest_vote_results_sqlx(pg: &PgPool) -> sqlx::Result<Vec<VoteResult>> {
    futures::future::join_all(
        get_latest_legislative_initiatives_sqlx(pg)
            .await?
            .into_iter()
            .map(|legis_init| async {
                Ok(VoteResult {
                    votes: get_votes_from_legis_init_sqlx(pg, legis_init.id).await?,
                    speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
                    topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
                    legislative_initiative: legis_init,
                })
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<VoteResult>>>()
}

pub async fn get_latest_vote_results_sqlx_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&LegisInitFilter>,
) -> sqlx::Result<(Vec<VoteResult>, i64)> {
    let (entries, entry_count) =
        get_latest_legis_inits_per_page(pg, page, page_elements, filter).await?;

    let entries = entries
        .into_iter()
        .map(|legis_init| async {
            Ok(VoteResult {
                votes: get_votes_from_legis_init_sqlx(pg, legis_init.id).await?,
                speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
                topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
                legislative_initiative: legis_init,
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(entries)
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<VoteResult>>>()
        .map(|x| (x, entry_count))
}

pub fn get_latest_vote_results(con: &mut PgConnection) -> QueryResult<Vec<VoteResult>> {
    get_latest_legislative_initiatives(con)?
        .into_iter()
        .map(|legis_init| {
            Ok(VoteResult {
                votes: get_votes_from_legis_init(con, legis_init.id)?,
                speeches: get_speeches_from_legis_init(con, legis_init.id)?,
                topics: vec![],
                legislative_initiative: legis_init,
            })
        })
        .collect::<QueryResult<Vec<VoteResult>>>()
}

pub async fn get_eurovoc_topics_from_legis_init(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<Topic>> {
    sqlx::query_as!(
        Topic,
        "select topic from eurovoc_topics_legis_init where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub async fn get_votes_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbVote>> {
    sqlx::query_as!(
        DbVote,
        "select * from votes where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub fn get_votes_from_legis_init(
    con: &mut PgConnection,
    legis_init_id: i32,
) -> QueryResult<Vec<DbVote>> {
    votes
        .filter(legislative_initiatives_id.eq(legis_init_id))
        .load::<DbVote>(con)
}

pub async fn get_speeches_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbSpeech>> {
    sqlx::query_as!(
        DbSpeech,
        "select * from speeches where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}
