use dataservice::db::models::DbLegislativeInitiativeQuery;
use somes_common_lib::AddonVoteResultFilter;
use sqlx::{PgPool, Postgres, QueryBuilder};

fn push_base_filter<'a>(
    query: &mut QueryBuilder<'a, Postgres>,
    filter: &'a AddonVoteResultFilter,
    is_finished: bool,
) {
    if is_finished {
        query.push(" li.accepted IS NOT NULL AND li.is_voteable_on");
    } else {
        query.push(
            " li.accepted IS NULL AND NOT li.has_reference AND li.ityp != 'EUBTG' AND li.is_voteable_on",
        );
    }

    if let Some(date_from) = filter.date_from {
        query
            .push(" AND li.nr_plenary_activity_date >= ")
            .push_bind(date_from);
    }

    if let Some(date_to) = filter.date_to {
        query
            .push(" AND li.nr_plenary_activity_date <= ")
            .push_bind(date_to);
    }

    if let Some(party_votes) = &filter.party_votes {
        for party_vote in party_votes {
            query.push(
                " AND EXISTS (
                    SELECT 1
                    FROM votes
                    WHERE votes.legislative_initiatives_id = li.id
                        AND votes.party = ",
            );
            query.push_bind(&party_vote.party);
            query.push(" AND votes.infavor = ");
            query.push_bind(party_vote.infavor);
            query.push(")");
        }
    }
}

pub async fn filtered_legislative_initiatives(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: &AddonVoteResultFilter,
    is_finished: bool,
) -> Result<(Vec<DbLegislativeInitiativeQuery>, i64), sqlx::Error> {
    let offset = page.max(0) * page_elements.max(0);
    let limit = page_elements.max(0);

    let mut entries_query =
        QueryBuilder::new("SELECT DISTINCT li.* FROM legislative_initiatives li WHERE");
    push_base_filter(&mut entries_query, filter, is_finished);
    entries_query
        .push(" ORDER BY li.nr_plenary_activity_date DESC OFFSET ")
        .push_bind(offset)
        .push(" LIMIT ")
        .push_bind(limit);

    let mut count_query =
        QueryBuilder::new("SELECT COUNT(DISTINCT li.id) FROM legislative_initiatives li WHERE");
    push_base_filter(&mut count_query, filter, is_finished);

    Ok((
        entries_query
            .build_query_as::<DbLegislativeInitiativeQuery>()
            .fetch_all(pg)
            .await?,
        count_query
            .build_query_scalar::<i64>()
            .fetch_one(pg)
            .await?,
    ))
}
