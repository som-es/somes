use sqlx::PgPool;

use super::*;

/// Mood values are averages of doubles, so they are compared approximately.
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected}, got {actual}"
    );
}

fn claims(id: i32) -> Claims {
    Claims::new(id, "delegate-name".to_string(), false, false)
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_extract_barometer_sqlx_aggregates_user_moods(pool: PgPool) {
    let barometer = extract_barometer_sqlx(&pool, "2024-25", 1)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(barometer.gov_prop_id, 1);
    assert_eq!(barometer.mood_id, 1);
    assert_close(barometer.auto_mood, 0.7);
    assert_eq!(barometer.pre_aggregated_user_mood, Some(0.4));

    assert_eq!(barometer.user_moods.len(), 3);
    for expected in [0.2, 0.4, 0.6] {
        assert!(
            barometer
                .user_moods
                .iter()
                .any(|user_mood| (user_mood - expected).abs() < 1e-9),
            "{} missing from {:?}",
            expected,
            barometer.user_moods
        );
    }
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_extract_barometer_sqlx_without_user_moods(pool: PgPool) {
    let barometer = extract_barometer_sqlx(&pool, "2024-25", 2)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(barometer.gov_prop_id, 2);
    assert_close(barometer.auto_mood, 0.1);
    assert_eq!(barometer.pre_aggregated_user_mood, None);
    assert!(barometer.user_moods.is_empty());
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_extract_barometer_sqlx_fails_for_proposal_without_mood(pool: PgPool) {
    let error = extract_barometer_sqlx(&pool, "2024-25", 3)
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        UserError::SqlFailure(sqlx::Error::RowNotFound)
    ));
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_mood_values_for_gov_prop_route_returns_barometer(pool: PgPool) {
    let Json(barometer) = mood_values_for_gov_prop(
        PgPoolConnection(pool.clone()),
        Path(("2024-25".to_string(), 1)),
    )
    .await
    .unwrap();
    let barometer = barometer.unwrap();

    assert_eq!(barometer.gov_prop_id, 1);
    assert_eq!(barometer.mood_id, 1);
    assert_close(barometer.auto_mood, 0.7);
    assert_eq!(barometer.user_moods.len(), 3);
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_add_mood_value_route_creates_mood_for_unrated_proposal(pool: PgPool) {
    let Json(barometer) = add_mood_value_route(
        PgPoolConnection(pool.clone()),
        claims(1),
        Path(("2024-25".to_string(), 3)),
        Json(AddMoodValue { user_mood: 0.5 }),
    )
    .await
    .unwrap();

    assert_eq!(barometer.gov_prop_id, 3);
    assert_close(barometer.auto_mood, 0.0);
    assert_close(barometer.pre_aggregated_user_mood.unwrap(), 0.5);
    assert_eq!(barometer.user_moods, vec![0.5]);

    // a mood and the link to the gov proposal were created
    let mood_id = barometer.mood_id;
    let gov_prop_mood_count = sqlx::query_scalar!(
        "select count(*) from gov_prop_mood where gov_prop_id = $1 and mood_id = $2",
        barometer.gov_prop_id,
        mood_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(gov_prop_mood_count, Some(1));
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_add_mood_value_route_updates_existing_user_mood(pool: PgPool) {
    // user 1 already rated the proposal with 0.2, the new value replaces it
    let Json(barometer) = add_mood_value_route(
        PgPoolConnection(pool.clone()),
        claims(1),
        Path(("2024-25".to_string(), 1)),
        Json(AddMoodValue { user_mood: 0.5 }),
    )
    .await
    .unwrap();

    assert_eq!(barometer.mood_id, 1);
    assert_eq!(barometer.user_moods.len(), 3);
    assert_close(barometer.pre_aggregated_user_mood.unwrap(), 0.5);

    // a second user adds its rating, the aggregation covers both users
    let Json(barometer) = add_mood_value_route(
        PgPoolConnection(pool.clone()),
        claims(2),
        Path(("2024-25".to_string(), 1)),
        Json(AddMoodValue { user_mood: 0.25 }),
    )
    .await
    .unwrap();

    assert_close(barometer.pre_aggregated_user_mood.unwrap(), 0.45);
    assert_eq!(barometer.user_moods.len(), 3);

    // no user mood was inserted twice
    let user_count = sqlx::query_scalar!(
        "select count(distinct user_id) from user_mood where mood_id = $1",
        1
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(user_count, Some(3));
}

#[sqlx::test(fixtures("fixtures/mood_barometer_base.sql"))]
async fn test_add_mood_value_route_fails_for_unknown_proposal(pool: PgPool) {
    let error = add_mood_value_route(
        PgPoolConnection(pool),
        claims(1),
        Path(("2024-25".to_string(), 99)),
        Json(AddMoodValue { user_mood: 0.5 }),
    )
    .await
    .unwrap_err();

    assert!(matches!(error, UserError::Custom(StatusCode::NOT_FOUND, _)));
}
