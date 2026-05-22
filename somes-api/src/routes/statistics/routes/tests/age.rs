use super::*;

fn create_test_base_data() -> Vec<AgeBase> {
    vec![
        AgeBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            age: 45,
            birthdate: Some(chrono::NaiveDate::from_ymd_opt(1980, 1, 1).unwrap()),
            legislative_period: Some("XXV".to_string()),
        },
        AgeBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            age: 35,
            birthdate: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
            legislative_period: Some("XXV".to_string()),
        },
        AgeBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            age: 55,
            birthdate: Some(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            legislative_period: Some("XXV".to_string()),
        },
        AgeBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            age: 42,
            birthdate: Some(chrono::NaiveDate::from_ymd_opt(1983, 1, 1).unwrap()),
            legislative_period: Some("XXVII".to_string()),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = AgeService::aggregate_by_party(base_data, true);

    // Verify Party X: avg = (45 + 35) / 2 = 40.0, min = 35, max = 45
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_age - 40.0).abs() < 0.001);
    assert_eq!(party_x.delegate_count, 2);
    assert_eq!(party_x.min_age, 35);
    assert_eq!(party_x.max_age, 45);

    // Verify Party Y: avg = (55 + 42) / 2 = 48.5, min = 42, max = 55
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_age - 48.5).abs() < 0.001);
    assert_eq!(party_y.delegate_count, 2);
    assert_eq!(party_y.min_age, 42);
    assert_eq!(party_y.max_age, 55);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = AgeService::aggregate_by_party(base_data, false);

    // When is_desc is false, results should be in ascending order
    assert!(results[0].average_age < results[1].average_age);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = AgeService::aggregate_by_gender(base_data, true);

    // Verify M: avg = (45 + 55) / 2 = 50.0, min = 45, max = 55
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_age - 50.0).abs() < 0.001);
    assert_eq!(male.delegate_count, 2);
    assert_eq!(male.min_age, 45);
    assert_eq!(male.max_age, 55);

    // Verify F: avg = (35 + 42) / 2 = 38.5, min = 35, max = 42
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_age - 38.5).abs() < 0.001);
    assert_eq!(female.delegate_count, 2);
    assert_eq!(female.min_age, 35);
    assert_eq!(female.max_age, 42);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = AgeService::aggregate_by_legis(base_data, true);

    // Verify 51: avg = (45 + 35 + 55) / 3 = 45.0, min = 35, max = 55
    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_age - 45.0).abs() < 0.001);
    assert_eq!(period_51.delegate_count, 3);
    assert_eq!(period_51.min_age, 35);
    assert_eq!(period_51.max_age, 55);

    // Verify 52: avg = 42.0, min = 42, max = 42
    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_age - 42.0).abs() < 0.001);
    assert_eq!(period_52.delegate_count, 1);
    assert_eq!(period_52.min_age, 42);
    assert_eq!(period_52.max_age, 42);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = AgeService::aggregate_by_age(base_data, true);

    // Verify 41-50: avg = (45 + 42) / 2 = 43.5, min = 42, max = 45
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_age - 43.5).abs() < 0.001);
    assert_eq!(age_41_50.delegate_count, 2);
    assert_eq!(age_41_50.min_age, 42);
    assert_eq!(age_41_50.max_age, 45);

    // Verify 31-40: avg = 35.0, min = 35, max = 35
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_age - 35.0).abs() < 0.001);
    assert_eq!(age_31_40.delegate_count, 1);
    assert_eq!(age_31_40.min_age, 35);
    assert_eq!(age_31_40.max_age, 35);

    // Verify 51-60: avg = 55.0, min = 55, max = 55
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_age - 55.0).abs() < 0.001);
    assert_eq!(age_51_60.delegate_count, 1);
    assert_eq!(age_51_60.min_age, 55);
    assert_eq!(age_51_60.max_age, 55);
}

#[tokio::test]
async fn test_get_base_data_applies_filters_and_computes_age() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_get_base_data_applies_filters_and_computes_age",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = AgeFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = AgeService::get_base_data(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert_eq!(delegate.age, 40);
    assert_eq!(
        delegate.birthdate,
        Some(chrono::NaiveDate::from_ymd_opt(1980, 1, 1).unwrap())
    );
    assert_eq!(delegate.legislative_period, Some("XXV".to_string()));
}

#[tokio::test]
async fn test_per_delegate_uses_latest_period_for_unfiltered_age() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_per_delegate_uses_latest_period_for_unfiltered_age",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = AgeFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = AgeService::per_delegate(&pool, &filter).await.unwrap();

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
    assert_eq!(delegate.age, 37);
}
