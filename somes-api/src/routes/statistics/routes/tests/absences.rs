use super::*;

fn create_test_base_data() -> Vec<AbsenceBase> {
    vec![
        AbsenceBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            total_absences: 10,
            total_sessions: 20,
            normalized_absences: 0.5,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        AbsenceBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            total_absences: 5,
            total_sessions: 20,
            normalized_absences: 0.25,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        AbsenceBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            total_absences: 15,
            total_sessions: 20,
            normalized_absences: 0.75,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        AbsenceBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            total_absences: 8,
            total_sessions: 20,
            normalized_absences: 0.4,
            legislative_period: Some("52".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = AbsenceService::aggregate_by_party(base_data, true, false);

    // Verify Party X: total_absences = 15, total_sessions = 40, normalized = 15/40 = 0.375
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert_eq!(party_x.total_absences, 15);
    assert_eq!(party_x.total_sessions, 40);
    assert!((party_x.normalized_absences - 0.375).abs() < 0.001);

    // Verify Party Y: total_absences = 23, total_sessions = 40, normalized = 23/40 = 0.575
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert_eq!(party_y.total_absences, 23);
    assert_eq!(party_y.total_sessions, 40);
    assert!((party_y.normalized_absences - 0.575).abs() < 0.001);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = AbsenceService::aggregate_by_party(base_data, false, false);

    assert!(results[0].total_absences < results[1].total_absences);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = AbsenceService::aggregate_by_gender(base_data, true, false);

    // Verify M: total_absences = 25, total_sessions = 40, normalized = 25/40 = 0.625
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert_eq!(male.total_absences, 25);
    assert_eq!(male.total_sessions, 40);
    assert!((male.normalized_absences - 0.625).abs() < 0.001);

    // Verify F: total_absences = 13, total_sessions = 40, normalized = 13/40 = 0.325
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert_eq!(female.total_absences, 13);
    assert_eq!(female.total_sessions, 40);
    assert!((female.normalized_absences - 0.325).abs() < 0.001);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = AbsenceService::aggregate_by_legis(base_data, true, false);

    // Verify 51: total_absences = 30, total_sessions = 60, normalized = 30/60 = 0.5
    let period_51 = results.iter().find(|r| r.category == "51").unwrap();
    assert_eq!(period_51.total_absences, 30);
    assert_eq!(period_51.total_sessions, 60);
    assert!((period_51.normalized_absences - 0.5).abs() < 0.001);

    // Verify 52: total_absences = 8, total_sessions = 20, normalized = 8/20 = 0.4
    let period_52 = results.iter().find(|r| r.category == "52").unwrap();
    assert_eq!(period_52.total_absences, 8);
    assert_eq!(period_52.total_sessions, 20);
    assert!((period_52.normalized_absences - 0.4).abs() < 0.001);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = AbsenceService::aggregate_by_age(base_data, true, false);

    // Verify 41-50: total_absences = 18, total_sessions = 40, normalized = 18/40 = 0.45
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert_eq!(age_41_50.total_absences, 18);
    assert_eq!(age_41_50.total_sessions, 40);
    assert!((age_41_50.normalized_absences - 0.45).abs() < 0.001);

    // Verify 31-40: total_absences = 5, total_sessions = 20, normalized = 5/20 = 0.25
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert_eq!(age_31_40.total_absences, 5);
    assert_eq!(age_31_40.total_sessions, 20);
    assert!((age_31_40.normalized_absences - 0.25).abs() < 0.001);

    // Verify 51-60: total_absences = 15, total_sessions = 20, normalized = 15/20 = 0.75
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert_eq!(age_51_60.total_absences, 15);
    assert_eq!(age_51_60.total_sessions, 20);
    assert!((age_51_60.normalized_absences - 0.75).abs() < 0.001);
}

#[test]
fn test_aggregate_by_party_sorts_by_normalized_score() {
    let base_data = vec![
        AbsenceBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            total_absences: 10,
            total_sessions: 100,
            normalized_absences: 0.1,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        AbsenceBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            total_absences: 2,
            total_sessions: 4,
            normalized_absences: 0.5,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
    ];

    let results = AbsenceService::aggregate_by_party(base_data, true, true);

    assert_eq!(results[0].category, "Party Y");
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_get_base_data_applies_filters_and_computes_absence_stats(pool: sqlx::PgPool) {
    let filter = AbsenceFilter {
        legis_period: Some("51".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = AbsenceService::get_base_data(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert_eq!(delegate.total_absences, 2);
    assert_eq!(delegate.total_sessions, 1);
    assert!((delegate.normalized_absences - 2.0).abs() < 0.001);
    assert_eq!(delegate.legislative_period, Some("51".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_per_legis_keeps_delegates_with_data_in_multiple_periods(pool: sqlx::PgPool) {
    let filter = AbsenceFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = AbsenceService::per_legis(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 3);

    let period_51 = results.iter().find(|r| r.category == "51").unwrap();
    assert_eq!(period_51.total_absences, 4);
    assert_eq!(period_51.total_sessions, 3);
    assert!((period_51.normalized_absences - (4.0 / 3.0)).abs() < 0.001);

    let period_52 = results.iter().find(|r| r.category == "52").unwrap();
    assert_eq!(period_52.total_absences, 4);
    assert_eq!(period_52.total_sessions, 3);
    assert!((period_52.normalized_absences - (4.0 / 3.0)).abs() < 0.001);

    let period_53 = results.iter().find(|r| r.category == "53").unwrap();
    assert_eq!(period_53.total_absences, 2);
    assert_eq!(period_53.total_sessions, 2);
    assert!((period_53.normalized_absences - 1.0).abs() < 0.001);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row(pool: sqlx::PgPool) {
    let filter = AbsenceFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = AbsenceService::per_delegate(&pool, &filter).await.unwrap();

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
    assert_eq!(delegate.total_absences, 3);
    assert_eq!(delegate.total_sessions, 2);
    assert!((delegate.normalized_absences - 1.5).abs() < 0.001);
}
