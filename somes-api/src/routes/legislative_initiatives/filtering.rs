use dataservice::db::models::DbLegislativeInitiativeQuery;
use somes_common_lib::LegisInitFilter;
use sqlx::PgPool;

pub async fn filtered_legislative_initiatives(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: &LegisInitFilter,
    is_finished: bool,
) -> Result<(Vec<DbLegislativeInitiativeQuery>, i64), sqlx::Error> {
    let default_where_clause = if is_finished {
        "accepted is not null"
    } else {
        "accepted is null and not has_reference"
    };
    let mut query =
        format!("SELECT DISTINCT * FROM legislative_initiatives WHERE {default_where_clause}");

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

    if filter.is_named_vote.is_some() {
        query.push_str(&format!(" AND voted_by_name = ${}", param_index));
        param_index += 1;
    }

    if filter.is_law.is_some() {
        query.push_str(&format!(" AND is_law = ${}", param_index));
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

    if let Some(accepted_value) = &filter.accepted {
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
    if let Some(is_named_vote) = &filter.is_named_vote {
        filtered_query = filtered_query.bind(is_named_vote);
        count_query = count_query.bind(is_named_vote);
    }

    if let Some(is_law) = &filter.is_law {
        filtered_query = filtered_query.bind(is_law);
        count_query = count_query.bind(is_law);
    }
    filtered_query = filtered_query
        .bind(page * page_elements)
        .bind(page_elements);
    Ok((
        filtered_query.fetch_all(pg).await?,
        count_query.fetch_one(pg).await?.0,
    ))
}
