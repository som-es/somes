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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use somes_common_lib::{AddonVoteResultFilter, PartyVote};
    use sqlx::{Postgres, QueryBuilder};

    use super::push_base_filter;

    fn filter_sql(filter: &AddonVoteResultFilter, is_finished: bool) -> String {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT li.id FROM legislative_initiatives li WHERE");
        push_base_filter(&mut query, filter, is_finished);
        query.sql().to_string()
    }

    #[test]
    fn finished_filter_requires_accepted_voteable_entries() {
        let sql = filter_sql(&AddonVoteResultFilter::default(), true);

        assert!(sql.contains("li.accepted IS NOT NULL"));
        assert!(sql.contains("li.is_voteable_on"));
        assert!(!sql.contains("li.accepted IS NULL"));
    }

    #[test]
    fn unfinished_filter_excludes_references_and_eubtg_entries() {
        let sql = filter_sql(&AddonVoteResultFilter::default(), false);

        assert!(sql.contains("li.accepted IS NULL"));
        assert!(sql.contains("NOT li.has_reference"));
        assert!(sql.contains("li.ityp != 'EUBTG'"));
        assert!(sql.contains("li.is_voteable_on"));
    }

    #[test]
    fn date_and_party_filters_are_parameterized() {
        let filter = AddonVoteResultFilter {
            date_from: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            date_to: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            party_votes: Some(vec![
                PartyVote {
                    party: "Gruene".to_string(),
                    infavor: true,
                },
                PartyVote {
                    party: "SPOe".to_string(),
                    infavor: false,
                },
            ]),
            ..Default::default()
        };

        let sql = filter_sql(&filter, true);

        assert!(sql.contains("li.nr_plenary_activity_date >= $1"));
        assert!(sql.contains("li.nr_plenary_activity_date <= $2"));
        assert!(sql.contains("votes.party = $3"));
        assert!(sql.contains("votes.infavor = $4"));
        assert!(sql.contains("votes.party = $5"));
        assert!(sql.contains("votes.infavor = $6"));
        assert!(!sql.contains("Gruene"));
        assert!(!sql.contains("SPOe"));
    }
}
