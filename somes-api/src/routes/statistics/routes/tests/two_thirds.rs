use super::*;
use sqlx::PgPool;

#[sqlx::test(fixtures("fixtures/statistics_base.sql", "fixtures/two_thirds.sql"))]
async fn party_positions_exclude_missing_votes_and_simple_majorities(pool: PgPool) {
    let Json(rows) = two_thirds_by_party(PgPoolConnection(pool.clone()), Json(None))
        .await
        .unwrap();
    let a = rows.iter().find(|r| r.category == "Test A").unwrap();
    assert_eq!(
        (a.positive, a.negative, a.other, a.unknown, a.total),
        (1, 1, 1, 1, 4)
    );
    assert!(!rows.iter().any(|r| r.category == "Absent"));
    let Json(filtered) = two_thirds_by_party(
        PgPoolConnection(pool),
        Json(Some(TwoThirdsFilter {
            legis_period: Some("TEST-II".into()),
        })),
    )
    .await
    .unwrap();
    let a = filtered.iter().find(|r| r.category == "Test A").unwrap();
    assert_eq!((a.positive, a.total), (0, 1));
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql", "fixtures/two_thirds.sql"))]
async fn topics_count_distinct_bills_per_topic_and_keep_open_outcomes(pool: PgPool) {
    let Json(rows) = two_thirds_by_topic(PgPoolConnection(pool.clone()), Json(None))
        .await
        .unwrap();
    let health = rows.iter().find(|r| r.category == "Health").unwrap();
    assert_eq!(
        (
            health.positive,
            health.negative,
            health.other,
            health.unknown,
            health.total
        ),
        (1, 1, 1, 1, 4)
    );
    let education = rows.iter().find(|r| r.category == "Education").unwrap();
    assert_eq!((education.positive, education.total), (1, 1));
    let Json(filtered) = two_thirds_by_topic(
        PgPoolConnection(pool),
        Json(Some(TwoThirdsFilter {
            legis_period: Some("TEST-II".into()),
        })),
    )
    .await
    .unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!((filtered[0].unknown, filtered[0].total), (1, 1));
}
