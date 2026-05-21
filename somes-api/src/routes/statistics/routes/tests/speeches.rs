use super::*;

fn create_test_base_data() -> Vec<SpeechBase> {
    vec![
        SpeechBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            total_speeches: 10,
            total_speech_time: 100,
            average_speech_time: 10.0,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        SpeechBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            total_speeches: 5,
            total_speech_time: 80,
            average_speech_time: 16.0,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        SpeechBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            total_speeches: 8,
            total_speech_time: 160,
            average_speech_time: 20.0,
            legislative_period: Some("51".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        SpeechBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            total_speeches: 12,
            total_speech_time: 120,
            average_speech_time: 10.0,
            legislative_period: Some("52".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();

    let results = SpeechService::aggregate_by_party(base_data, true, "speechtime", false);

    // Verify Party X: speeches = 15, speech_time = 180, avg = (10 + 16) / 2 = 13
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert_eq!(party_x.total_speeches, 15);
    assert_eq!(party_x.total_speech_time, 180);
    assert!((party_x.average_speech_time - 13.0).abs() < 0.001);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: speeches = 20, speech_time = 280, avg = (20 + 10) / 2 = 15
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert_eq!(party_y.total_speeches, 20);
    assert_eq!(party_y.total_speech_time, 280);
    assert!((party_y.average_speech_time - 15.0).abs() < 0.001);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_party(base_data, false, "speechtime", false);

    assert!(results[0].total_speech_time < results[1].total_speech_time);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_gender(base_data, true, "speechtime", false);

    // Verify M: speeches = 18, speech_time = 260, avg = (10 + 20) / 2 = 15
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert_eq!(male.total_speeches, 18);
    assert_eq!(male.total_speech_time, 260);
    assert!((male.average_speech_time - 15.0).abs() < 0.001);
    assert_eq!(male.delegate_count, 2);

    // Verify F: speeches = 17, speech_time = 200, avg = (16 + 10) / 2 = 13
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert_eq!(female.total_speeches, 17);
    assert_eq!(female.total_speech_time, 200);
    assert!((female.average_speech_time - 13.0).abs() < 0.001);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_legis(base_data, true, "speechtime", false);

    // Verify 51: speeches = 23, speech_time = 340, avg = (10 + 16 + 20) / 3
    let period_51 = results.iter().find(|r| r.category == "51").unwrap();
    assert_eq!(period_51.total_speeches, 23);
    assert_eq!(period_51.total_speech_time, 340);
    assert!((period_51.average_speech_time - 15.3333333).abs() < 0.001);
    assert_eq!(period_51.delegate_count, 3);

    // Verify 52: speeches = 12, speech_time = 120, avg = 10
    let period_52 = results.iter().find(|r| r.category == "52").unwrap();
    assert_eq!(period_52.total_speeches, 12);
    assert_eq!(period_52.total_speech_time, 120);
    assert!((period_52.average_speech_time - 10.0).abs() < 0.001);
    assert_eq!(period_52.delegate_count, 1);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_age(base_data, true, "speechtime", false);

    // Verify 41-50: speeches = 22, speech_time = 220, avg = (10 + 10) / 2
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert_eq!(age_41_50.total_speeches, 22);
    assert_eq!(age_41_50.total_speech_time, 220);
    assert!((age_41_50.average_speech_time - 10.0).abs() < 0.001);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: speeches = 5, speech_time = 80, avg = 16
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert_eq!(age_31_40.total_speeches, 5);
    assert_eq!(age_31_40.total_speech_time, 80);
    assert!((age_31_40.average_speech_time - 16.0).abs() < 0.001);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: speeches = 8, speech_time = 160, avg = 20
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert_eq!(age_51_60.total_speeches, 8);
    assert_eq!(age_51_60.total_speech_time, 160);
    assert!((age_51_60.average_speech_time - 20.0).abs() < 0.001);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[test]
fn test_aggregate_by_party_sorts_by_total_speeches() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_party(base_data, true, "total_speeches", false);

    assert_eq!(results[0].category, "Party Y");
}

#[test]
fn test_aggregate_by_party_sorts_by_average_speech_time() {
    let base_data = create_test_base_data();
    let results = SpeechService::aggregate_by_party(base_data, true, "speechtime", true);

    assert_eq!(results[0].category, "Party Y");
}
#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_get_base_data_applies_filters_and_computes_speech_stats(pool: sqlx::PgPool) {
    let filter = SpeechFilter {
        legis_period: Some("51".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = SpeechService::get_base_data(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert_eq!(delegate.total_speeches, 2);
    assert_eq!(delegate.total_speech_time, 180);
    assert!((delegate.average_speech_time - 90.0).abs() < 0.001);
    assert_eq!(delegate.legislative_period, Some("51".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_speech_handlers_override_speech_type(pool: sqlx::PgPool) {
    let speechtime_filter = SpeechFilter {
        legis_period: Some("51".to_string()),
        is_desc: true,
        speech_type: "total_speeches".to_string(),
        ..Default::default()
    };

    let Json(speechtime_results) = speechtime_per_delegate(
        PgPoolConnection(pool.clone()),
        Json(Some(speechtime_filter)),
    )
    .await
    .unwrap();

    assert_eq!(speechtime_results[0].delegate_name, "Delegate B");
    assert_eq!(speechtime_results[0].total_speech_time, 240);

    let total_speeches_filter = SpeechFilter {
        legis_period: Some("51".to_string()),
        is_desc: true,
        speech_type: "speechtime".to_string(),
        ..Default::default()
    };

    let Json(total_speech_results) =
        total_speeches_per_delegate(PgPoolConnection(pool), Json(Some(total_speeches_filter)))
            .await
            .unwrap();

    assert_eq!(total_speech_results[0].delegate_name, "Delegate A");
    assert_eq!(total_speech_results[0].total_speeches, 2);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row(pool: sqlx::PgPool) {
    let filter = SpeechFilter {
        is_desc: true,
        speech_type: "speechtime".to_string(),
        ..Default::default()
    };

    let results = SpeechService::per_delegate(&pool, &filter).await.unwrap();

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
    assert_eq!(delegate.total_speeches, 2);
    assert_eq!(delegate.total_speech_time, 300);
    assert!((delegate.average_speech_time - 150.0).abs() < 0.001);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_government_member_displays_party_but_keeps_filter_bucket(pool: sqlx::PgPool) {
    let filter = SpeechFilter {
        legis_period: Some("53".to_string()),
        party: Some("Regierungsmitglied".to_string()),
        ..Default::default()
    };

    let results = SpeechService::per_delegate(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].delegate_name, "Delegate G");
    assert_eq!(results[0].delegate_party, "Party X");
    assert_eq!(results[0].delegate_filter_party, "Regierungsmitglied");
    assert_eq!(results[0].total_speech_time, 60);

    let party_filter = SpeechFilter {
        legis_period: Some("53".to_string()),
        party: Some("Party X".to_string()),
        ..Default::default()
    };

    let party_results = SpeechService::per_delegate(&pool, &party_filter)
        .await
        .unwrap();

    assert!(party_results.is_empty());
}
