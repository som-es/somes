use sqlx::PgPool;

use super::*;

fn create_test_base_data() -> Vec<ComplexityBase> {
    vec![
        ComplexityBase {
            delegate_id: 1,
            latest_activity_date: Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: Some("M".to_string()),
            complexity_score: 1.2,
            total_proposals: 10,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        ComplexityBase {
            delegate_id: 2,
            latest_activity_date: Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: Some("F".to_string()),
            complexity_score: 1.0,
            total_proposals: 5,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        ComplexityBase {
            delegate_id: 3,
            latest_activity_date: Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: Some("M".to_string()),
            complexity_score: 1.3,
            total_proposals: 8,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        ComplexityBase {
            delegate_id: 4,
            latest_activity_date: Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: Some("F".to_string()),
            complexity_score: 1.1,
            total_proposals: 12,
            legislative_period: Some("XXVII".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_party(base_data, true);

    // Verify Party X: (1.2 + 1.0) / 2 = 1.1, total_proposals = 15, delegate_count = 2
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_complexity - 1.1).abs() < 0.001);
    assert_eq!(party_x.total_proposals, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: (1.3 + 1.1) / 2 = 1.2, total_proposals = 20, delegate_count = 2
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_complexity - 1.2).abs() < 0.001);
    assert_eq!(party_y.total_proposals, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_party(base_data, false);

    // When is_desc is false, results should be in ascending order
    assert!(results[0].average_complexity < results[1].average_complexity);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_gender(base_data, true);

    // Verify M: (1.2 + 1.3) / 2 = 1.25, total_proposals = 18, delegate_count = 2
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_complexity - 1.25).abs() < 0.001);
    assert_eq!(male.total_proposals, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: (1.0 + 1.1) / 2 = 1.05, total_proposals = 17, delegate_count = 2
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_complexity - 1.05).abs() < 0.001);
    assert_eq!(female.total_proposals, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_legis(base_data, true);

    // Verify 51: (1.2 + 1.0 + 1.3) / 3 = 1.166..., total_proposals = 23, delegate_count = 3
    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_complexity - 1.1666667).abs() < 0.001);
    assert_eq!(period_51.total_proposals, 23);
    assert_eq!(period_51.delegate_count, 3);

    // Verify 52: 1.1, total_proposals = 12, delegate_count = 1
    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_complexity - 1.1).abs() < 0.001);
    assert_eq!(period_52.total_proposals, 12);
    assert_eq!(period_52.delegate_count, 1);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_age(base_data, true);

    // Verify 41-50: (1.2 + 1.1) / 2 = 1.15, total_proposals = 22, delegate_count = 2
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_complexity - 1.15).abs() < 0.001);
    assert_eq!(age_41_50.total_proposals, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: 1.0, total_proposals = 5, delegate_count = 1
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_complexity - 1.0).abs() < 0.001);
    assert_eq!(age_31_40.total_proposals, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: 1.3, total_proposals = 8, delegate_count = 1
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_complexity - 1.3).abs() < 0.001);
    assert_eq!(age_51_60.total_proposals, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_get_base_data_applies_filters_and_computes_complexity_stats(pool: PgPool) {
    let filter = ComplexityFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = ComplexityService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, Some("M".to_string()));
    assert!((delegate.complexity_score - 1.25).abs() < 0.001);
    assert_eq!(delegate.total_proposals, 2);
    assert_eq!(delegate.legislative_period, Some("XXV".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_legis_keeps_delegates_with_data_in_multiple_periods(pool: PgPool) {
    let filter = ComplexityFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = ComplexityService::per_legis(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 3);

    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_complexity - 1.225).abs() < 0.001);
    assert_eq!(period_51.total_proposals, 3);
    assert_eq!(period_51.delegate_count, 2);

    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_complexity - 1.05).abs() < 0.001);
    assert_eq!(period_52.total_proposals, 4);
    assert_eq!(period_52.delegate_count, 4);

    let period_53 = results.iter().find(|r| r.category == "XXVIII").unwrap();
    assert!((period_53.average_complexity - 1.225).abs() < 0.001);
    assert_eq!(period_53.total_proposals, 2);
    assert_eq!(period_53.delegate_count, 2);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row(pool: PgPool) {
    let filter = ComplexityFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = ComplexityService::per_delegate(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(
        results
            .iter()
            .filter(|r| r.delegate_name == "Delegate D")
            .count(),
        1
    );

    let delegate = results
        .iter()
        .find(|r| r.delegate_name == "Delegate D")
        .unwrap();
    assert_eq!(delegate.delegate_party, "Party Y");
    assert!((delegate.complexity_score - 1.1).abs() < 0.001);
    assert_eq!(delegate.total_proposals, 2);
}

#[test]
fn test_groups_merge_periods_before_averaging_people() {
    let mut rows = create_test_base_data();
    let mut later = rows[0].clone();
    later.legislative_period = Some("XXVIII".into());
    later.latest_activity_date = Some(chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    later.complexity_score = 1.0;
    later.total_proposals = 30;
    later.delegate_age_bucket = "51-60".into();
    rows.push(later);

    let parties = ComplexityService::aggregate_by_party(rows.clone(), true);
    let party = parties.iter().find(|r| r.category == "Party X").unwrap();
    assert_eq!(party.delegate_count, 2);
    assert_eq!(party.total_proposals, 45);
    assert!((party.average_complexity - 1.025).abs() < 1e-12);

    let genders = ComplexityService::aggregate_by_gender(rows.clone(), true);
    let male = genders.iter().find(|r| r.category == "M").unwrap();
    assert_eq!(male.delegate_count, 2);
    assert!((male.average_complexity - 1.175).abs() < 1e-12);

    let ages = ComplexityService::aggregate_by_age(rows, true);
    let older = ages.iter().find(|r| r.category == "51-60").unwrap();
    assert_eq!(older.delegate_count, 2);
    assert!((older.average_complexity - 1.175).abs() < 1e-12);
    assert_eq!(ages.iter().map(|r| r.delegate_count).sum::<i64>(), 4);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_overlapping_mandates_and_duplicate_links_do_not_duplicate_initiatives(pool: PgPool) {
    sqlx::raw_sql(
        "INSERT INTO mandates VALUES
        (20, 1, NULL, false, true, '2020-01-02', NULL),
        (21, 1, 'Party X', true, false, '2020-01-02', NULL);
        INSERT INTO proposal_delegates VALUES ('p2', 1, false);",
    )
    .execute(&pool)
    .await
    .unwrap();
    let filter = ComplexityFilter {
        legis_period: Some("XXV".into()),
        ..Default::default()
    };
    let rows = ComplexityService::per_delegate(&pool, &filter)
        .await
        .unwrap();
    let person = rows
        .iter()
        .find(|r| r.delegate_name == "Delegate A")
        .unwrap();
    assert_eq!(person.total_proposals, 2);
    assert!((person.complexity_score - 1.25).abs() < 1e-12);
    assert_eq!(person.delegate_filter_party, "Party X");
    let groups = ComplexityService::per_party(&pool, &filter).await.unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].delegate_count, 2);
    assert_eq!(groups[0].total_proposals, 3);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_party_switch_in_one_period_counts_person_once_and_uses_latest_affiliation(
    pool: PgPool,
) {
    sqlx::raw_sql(
        "UPDATE mandates SET end_date = '2020-06-30' WHERE id = 4;
        UPDATE mandates SET start_date = '2020-07-01' WHERE id = 5;
        INSERT INTO proposals VALUES
            ('p10', 'I', 'XXV', '2020-08-01'), ('p11', 'I', 'XXV', '2020-09-01');
        INSERT INTO proposal_delegates VALUES ('p10', 4, false), ('p11', 4, false);",
    )
    .execute(&pool)
    .await
    .unwrap();
    let filter = ComplexityFilter {
        legis_period: Some("XXV".into()),
        ..Default::default()
    };
    let periods = ComplexityService::per_legis(&pool, &filter).await.unwrap();
    assert_eq!(periods.len(), 1);
    assert_eq!(periods[0].delegate_count, 2);
    assert_eq!(periods[0].total_proposals, 5);
    let expected = (1.25 + (1.2 + 1.3 + 1.3) / 3.0) / 2.0;
    assert!((periods[0].average_complexity - expected).abs() < 1e-12);
    let people = ComplexityService::per_delegate(&pool, &filter)
        .await
        .unwrap();
    let person = people
        .iter()
        .find(|r| r.delegate_name == "Delegate D")
        .unwrap();
    assert_eq!(person.delegate_filter_party, "Party Y");
    assert_eq!(person.total_proposals, 3);
    let parties = ComplexityService::per_party(&pool, &filter).await.unwrap();
    let old_party = parties.iter().find(|r| r.category == "Party X").unwrap();
    let new_party = parties.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((old_party.average_complexity - 1.225).abs() < 1e-12);
    assert!((new_party.average_complexity - 1.3).abs() < 1e-12);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_people_with_identical_names_stay_separate(pool: PgPool) {
    sqlx::raw_sql(
        "INSERT INTO delegates VALUES (20, 'Delegate A', 'M', '1980-01-01', 'Party X');
        INSERT INTO mandates VALUES (20, 20, 'Party X', true, false, '2019-01-01', NULL);
        INSERT INTO proposal_delegates VALUES ('p1', 20, false);",
    )
    .execute(&pool)
    .await
    .unwrap();
    let rows = ComplexityService::per_delegate(&pool, &ComplexityFilter::default())
        .await
        .unwrap();
    assert_eq!(
        rows.iter()
            .filter(|r| r.delegate_name == "Delegate A")
            .count(),
        2
    );
}
