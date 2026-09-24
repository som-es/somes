//! Asserts that every `#[utoipa::path]` literal still matches the route constant the
//! endpoint was registered with, and that both parliament mounts are mirrored.
//! Regenerate it whenever a route constant or a documented path changes.

use somes_api::server::openapi_document;
use somes_common_lib::{
    ABSENCES_PER_AGE, ABSENCES_PER_DELEGATE, ABSENCES_PER_GENDER, ABSENCES_PER_LEGIS,
    ABSENCES_PER_PARTY, ACTIVITY_PER_AGE, ACTIVITY_PER_DELEGATE, ACTIVITY_PER_GENDER,
    ACTIVITY_PER_LEGIS, ACTIVITY_PER_PARTY, ADD_QUIZ, AGE_OF_DELEGATES, AGE_PER_GENDER,
    AGE_PER_LEGIS, AGE_PER_PARTY, ALL_ACTIVE, ALL_AT_DATE, ALL_AT_DATE_WITH_SEAT_INFO, ALL_GPS,
    CALL_TO_ORDERS_PER_AGE, CALL_TO_ORDERS_PER_GENDER, CALL_TO_ORDERS_PER_LEGIS,
    CALL_TO_ORDERS_PER_PARTY, COALITION_PARTIES_PER_GP, COMPLEXITY_AT_AGE, COMPLEXITY_PER_DELEGATE,
    COMPLEXITY_PER_GENDER, COMPLEXITY_PER_LEGIS, COMPLEXITY_PER_PARTY, DELEGATE, DEPARTMENTS,
    DEPARTMENTS_PER_GP, DIVISION_ACCURACY_SCORE_PER_AGE, DIVISION_ACCURACY_SCORE_PER_DELEGATE,
    DIVISION_ACCURACY_SCORE_PER_GENDER, DIVISION_ACCURACY_SCORE_PER_LEGIS,
    DIVISION_ACCURACY_SCORE_PER_PARTY, EUROVOC_TOPICS, EXTEND, GOV_PROPOSALS, ID,
    IS_AUTHORITARIAN_PER_AGE, IS_AUTHORITARIAN_PER_DELEGATE, IS_AUTHORITARIAN_PER_GENDER,
    IS_AUTHORITARIAN_PER_LEGIS, IS_AUTHORITARIAN_PER_PARTY, IS_LEFT_PER_AGE, IS_LEFT_PER_DELEGATE,
    IS_LEFT_PER_GENDER, IS_LEFT_PER_LEGIS, IS_LEFT_PER_PARTY, IS_LIBERAL_PER_AGE,
    IS_LIBERAL_PER_DELEGATE, IS_LIBERAL_PER_GENDER, IS_LIBERAL_PER_LEGIS, IS_LIBERAL_PER_PARTY,
    IS_RIGHT_PER_AGE, IS_RIGHT_PER_DELEGATE, IS_RIGHT_PER_GENDER, IS_RIGHT_PER_LEGIS,
    IS_RIGHT_PER_PARTY, LATEST, LATEST_SESSION_ACTIVITY_OVERVIEW,
    LEGISLATIVE_INITIATIVES_WITHOUT_SIMPLE_MAJORITY, LIVE, LOGIN_ROUTE, NEXT_PLENAR_DATE, PARTIES,
    PARTIES_AT_GP, PARTIES_PER_GP, PLENAR_DATES, PLENARY_SESSIONS_PER_GP,
    POLITICAL_SPECTRUM_PER_AGE, POLITICAL_SPECTRUM_PER_DELEGATE, POLITICAL_SPECTRUM_PER_GENDER,
    POLITICAL_SPECTRUM_PER_PARTY, PUSH_SETTINGS, PUSH_TOKEN, PUSH_TOKENS, QUIZZES, RENEW_TOKEN,
    SEARCH, SEATS, SEND_MAIL_INFO, SPEECHES_PER_PAGE_ROUTE, SPEECHTIME_PER_AGE,
    SPEECHTIME_PER_DELEGATE, SPEECHTIME_PER_GENDER, SPEECHTIME_PER_LEGIS, SPEECHTIME_PER_PARTY,
    TOPIC_SELECTION, TOPICS, TOTAL_SPEECHES_PER_AGE, TOTAL_SPEECHES_PER_DELEGATE,
    TOTAL_SPEECHES_PER_GENDER, TOTAL_SPEECHES_PER_LEGIS, TOTAL_SPEECHES_PER_PARTY, USER,
    VOTE_RESULT, VOTES_TOGETHER, WALO_QUESTIONS,
};
use std::collections::HashSet;

const PARLIAMENT_ROUTES: &[(&str, &str)] = &[
    ("", "/all_gps"),
    ("", "/coalition_parties_per_gp"),
    ("", "/departments"),
    ("", "/departments_per_gp"),
    ("", "/eurovoc_topics"),
    ("", "/next_plenar_date"),
    ("", "/orientation_questions"),
    ("", "/parties"),
    ("", "/parties_at_gp"),
    ("", "/parties_per_gp"),
    ("", "/plenar_dates"),
    ("", "/plenary_sessions_per_gp"),
    ("", "/save_email"),
    ("", "/seats"),
    ("", "/topics"),
    ("", ALL_GPS),
    ("", COALITION_PARTIES_PER_GP),
    ("", DEPARTMENTS),
    ("", DEPARTMENTS_PER_GP),
    ("", EUROVOC_TOPICS),
    ("", NEXT_PLENAR_DATE),
    ("", PARTIES),
    ("", PARTIES_AT_GP),
    ("", PARTIES_PER_GP),
    ("", PLENARY_SESSIONS_PER_GP),
    ("", PLENAR_DATES),
    ("", SEATS),
    ("", TOPICS),
    ("/v1/decrees", "/ris_id/{ris_id}"),
    ("/v1/delegates", "/all_active"),
    ("/v1/delegates", "/all_at_date"),
    ("/v1/delegates", "/all_at_date_with_seat_info"),
    ("/v1/delegates", "/extend/{id}"),
    ("/v1/delegates", "/speeches_per_page"),
    ("/v1/delegates", ALL_ACTIVE),
    ("/v1/delegates", ALL_AT_DATE),
    ("/v1/delegates", ALL_AT_DATE_WITH_SEAT_INFO),
    ("/v1/delegates", EXTEND),
    ("/v1/delegates", SPEECHES_PER_PAGE_ROUTE),
    ("/v1/delegates/gov_officials", "/gov_proposals/{id}"),
    ("/v1/delegates/gov_officials", GOV_PROPOSALS),
    ("/v1/delegates/interjections", "/made"),
    ("/v1/delegates/interjections", "/received"),
    ("/v1/delegates/parliament_qa", "/answers"),
    ("/v1/delegates/parliament_qa", "/inquiries"),
    ("/v1/delegates/political_analysis", "/political_position"),
    ("/v1/events", "/"),
    ("/v1/events", "/create"),
    ("/v1/events", "/update"),
    ("/v1/gov_proposals", "/{gp}/{inr}"),
    ("/v1/gov_proposals/{gp}/{inr}/mood", "/user"),
    ("/v1/gov_proposals/{gp}/{inr}/mood", USER),
    ("/v1/sitemap", "/decrees"),
    ("/v1/sitemap", "/gov_proposals"),
    ("/v1/sitemap", "/questions"),
    ("/v1/sitemap", "/summary"),
    ("/v1/sitemap", "/vote_results"),
    ("/v1/speeches", "/{speech_id}"),
    ("/v1/statistics", "/absences_per_age"),
    ("/v1/statistics", "/absences_per_delegate"),
    ("/v1/statistics", "/absences_per_gender"),
    ("/v1/statistics", "/absences_per_legis"),
    ("/v1/statistics", "/absences_per_party"),
    ("/v1/statistics", "/activity_per_age"),
    ("/v1/statistics", "/activity_per_delegate"),
    ("/v1/statistics", "/activity_per_gender"),
    ("/v1/statistics", "/activity_per_legis"),
    ("/v1/statistics", "/activity_per_party"),
    ("/v1/statistics", "/age_of_delegates"),
    ("/v1/statistics", "/age_per_age"),
    ("/v1/statistics", "/age_per_gender"),
    ("/v1/statistics", "/age_per_legis"),
    ("/v1/statistics", "/age_per_party"),
    ("/v1/statistics", "/call_to_orders_per_age"),
    ("/v1/statistics", "/call_to_orders_per_gender"),
    ("/v1/statistics", "/call_to_orders_per_legis"),
    ("/v1/statistics", "/call_to_orders_per_party"),
    ("/v1/statistics", "/complexity_at_age"),
    ("/v1/statistics", "/complexity_per_delegate"),
    ("/v1/statistics", "/complexity_per_gender"),
    ("/v1/statistics", "/complexity_per_legis"),
    ("/v1/statistics", "/complexity_per_party"),
    ("/v1/statistics", "/division_accuracy_score_per_age"),
    ("/v1/statistics", "/division_accuracy_score_per_gender"),
    ("/v1/statistics", "/division_accuracy_score_per_legis"),
    ("/v1/statistics", "/division_accuracy_score_per_party"),
    ("/v1/statistics", "/division_accuracy_score_per_delegate"),
    ("/v1/statistics", "/is_authoritarian_per_age"),
    ("/v1/statistics", "/is_authoritarian_per_delegate"),
    ("/v1/statistics", "/is_authoritarian_per_gender"),
    ("/v1/statistics", "/is_authoritarian_per_legis"),
    ("/v1/statistics", "/is_authoritarian_per_party"),
    ("/v1/statistics", "/is_left_per_age"),
    ("/v1/statistics", "/is_left_per_delegate"),
    ("/v1/statistics", "/is_left_per_gender"),
    ("/v1/statistics", "/is_left_per_legis"),
    ("/v1/statistics", "/is_left_per_party"),
    ("/v1/statistics", "/is_liberal_per_age"),
    ("/v1/statistics", "/is_liberal_per_delegate"),
    ("/v1/statistics", "/is_liberal_per_gender"),
    ("/v1/statistics", "/is_liberal_per_legis"),
    ("/v1/statistics", "/is_liberal_per_party"),
    ("/v1/statistics", "/is_right_per_age"),
    ("/v1/statistics", "/is_right_per_delegate"),
    ("/v1/statistics", "/is_right_per_gender"),
    ("/v1/statistics", "/is_right_per_legis"),
    ("/v1/statistics", "/is_right_per_party"),
    ("/v1/statistics", "/latest_session_activity_overview"),
    (
        "/v1/statistics",
        "/legislative_initiatives_without_simple_majority",
    ),
    ("/v1/statistics", "/political_spectrum_per_age"),
    ("/v1/statistics", "/political_spectrum_per_delegate"),
    ("/v1/statistics", "/political_spectrum_per_gender"),
    ("/v1/statistics", "/political_spectrum_per_party"),
    ("/v1/statistics", "/speechtime_per_age"),
    ("/v1/statistics", "/speechtime_per_delegate"),
    ("/v1/statistics", "/speechtime_per_gender"),
    ("/v1/statistics", "/speechtime_per_legis"),
    ("/v1/statistics", "/speechtime_per_party"),
    ("/v1/statistics", "/total_speeches_per_age"),
    ("/v1/statistics", "/total_speeches_per_delegate"),
    ("/v1/statistics", "/total_speeches_per_gender"),
    ("/v1/statistics", "/total_speeches_per_legis"),
    ("/v1/statistics", "/total_speeches_per_party"),
    ("/v1/statistics", "/votes_together"),
    ("/v1/statistics", ABSENCES_PER_AGE),
    ("/v1/statistics", ABSENCES_PER_DELEGATE),
    ("/v1/statistics", ABSENCES_PER_GENDER),
    ("/v1/statistics", ABSENCES_PER_LEGIS),
    ("/v1/statistics", ABSENCES_PER_PARTY),
    ("/v1/statistics", ACTIVITY_PER_AGE),
    ("/v1/statistics", ACTIVITY_PER_DELEGATE),
    ("/v1/statistics", ACTIVITY_PER_GENDER),
    ("/v1/statistics", ACTIVITY_PER_LEGIS),
    ("/v1/statistics", ACTIVITY_PER_PARTY),
    ("/v1/statistics", AGE_OF_DELEGATES),
    ("/v1/statistics", AGE_PER_GENDER),
    ("/v1/statistics", AGE_PER_LEGIS),
    ("/v1/statistics", AGE_PER_PARTY),
    ("/v1/statistics", CALL_TO_ORDERS_PER_AGE),
    ("/v1/statistics", CALL_TO_ORDERS_PER_GENDER),
    ("/v1/statistics", CALL_TO_ORDERS_PER_LEGIS),
    ("/v1/statistics", CALL_TO_ORDERS_PER_PARTY),
    ("/v1/statistics", COMPLEXITY_AT_AGE),
    ("/v1/statistics", COMPLEXITY_PER_DELEGATE),
    ("/v1/statistics", COMPLEXITY_PER_GENDER),
    ("/v1/statistics", COMPLEXITY_PER_LEGIS),
    ("/v1/statistics", COMPLEXITY_PER_PARTY),
    ("/v1/statistics", DIVISION_ACCURACY_SCORE_PER_AGE),
    ("/v1/statistics", DIVISION_ACCURACY_SCORE_PER_DELEGATE),
    ("/v1/statistics", DIVISION_ACCURACY_SCORE_PER_GENDER),
    ("/v1/statistics", DIVISION_ACCURACY_SCORE_PER_LEGIS),
    ("/v1/statistics", DIVISION_ACCURACY_SCORE_PER_PARTY),
    ("/v1/statistics", IS_AUTHORITARIAN_PER_AGE),
    ("/v1/statistics", IS_AUTHORITARIAN_PER_DELEGATE),
    ("/v1/statistics", IS_AUTHORITARIAN_PER_GENDER),
    ("/v1/statistics", IS_AUTHORITARIAN_PER_LEGIS),
    ("/v1/statistics", IS_AUTHORITARIAN_PER_PARTY),
    ("/v1/statistics", IS_LEFT_PER_AGE),
    ("/v1/statistics", IS_LEFT_PER_DELEGATE),
    ("/v1/statistics", IS_LEFT_PER_GENDER),
    ("/v1/statistics", IS_LEFT_PER_LEGIS),
    ("/v1/statistics", IS_LEFT_PER_PARTY),
    ("/v1/statistics", IS_LIBERAL_PER_AGE),
    ("/v1/statistics", IS_LIBERAL_PER_DELEGATE),
    ("/v1/statistics", IS_LIBERAL_PER_GENDER),
    ("/v1/statistics", IS_LIBERAL_PER_LEGIS),
    ("/v1/statistics", IS_LIBERAL_PER_PARTY),
    ("/v1/statistics", IS_RIGHT_PER_AGE),
    ("/v1/statistics", IS_RIGHT_PER_DELEGATE),
    ("/v1/statistics", IS_RIGHT_PER_GENDER),
    ("/v1/statistics", IS_RIGHT_PER_LEGIS),
    ("/v1/statistics", IS_RIGHT_PER_PARTY),
    ("/v1/statistics", LATEST_SESSION_ACTIVITY_OVERVIEW),
    (
        "/v1/statistics",
        LEGISLATIVE_INITIATIVES_WITHOUT_SIMPLE_MAJORITY,
    ),
    ("/v1/statistics", POLITICAL_SPECTRUM_PER_AGE),
    ("/v1/statistics", POLITICAL_SPECTRUM_PER_DELEGATE),
    ("/v1/statistics", POLITICAL_SPECTRUM_PER_GENDER),
    ("/v1/statistics", POLITICAL_SPECTRUM_PER_PARTY),
    ("/v1/statistics", SPEECHTIME_PER_AGE),
    ("/v1/statistics", SPEECHTIME_PER_DELEGATE),
    ("/v1/statistics", SPEECHTIME_PER_GENDER),
    ("/v1/statistics", SPEECHTIME_PER_LEGIS),
    ("/v1/statistics", SPEECHTIME_PER_PARTY),
    ("/v1/statistics", TOTAL_SPEECHES_PER_AGE),
    ("/v1/statistics", TOTAL_SPEECHES_PER_DELEGATE),
    ("/v1/statistics", TOTAL_SPEECHES_PER_GENDER),
    ("/v1/statistics", TOTAL_SPEECHES_PER_LEGIS),
    ("/v1/statistics", TOTAL_SPEECHES_PER_PARTY),
    ("/v1/statistics", VOTES_TOGETHER),
    ("/v1/user", "/anonymize_email"),
    ("/v1/user", "/change_email"),
    ("/v1/user", "/delete"),
    ("/v1/user", "/init"),
    ("/v1/user", "/mcp"),
    ("/v1/user", "/send_mail_info"),
    ("/v1/user", "/topic_selection"),
    ("/v1/user", "/verify_email_change"),
    ("/v1/user", LOGIN_ROUTE),
    ("/v1/user", RENEW_TOKEN),
    ("/v1/user", SEND_MAIL_INFO),
    ("/v1/user", TOPIC_SELECTION),
    ("/v1/user/bookmark", "/delegate"),
    ("/v1/user/bookmark", "/vote_result"),
    ("/v1/user/bookmark", DELEGATE),
    ("/v1/user/bookmark", VOTE_RESULT),
    ("/v1/user/push_notifications", "/settings"),
    ("/v1/user/push_notifications", "/token"),
    ("/v1/user/push_notifications", "/tokens"),
    ("/v1/user/push_notifications", PUSH_SETTINGS),
    ("/v1/user/push_notifications", PUSH_TOKEN),
    ("/v1/user/push_notifications", PUSH_TOKENS),
    ("/v1/volksbg", "/weeks"),
    ("/v1/vote_results", "/id/{id}"),
    ("/v1/vote_results", "/latest"),
    ("/v1/vote_results", "/live"),
    ("/v1/vote_results", "/search"),
    ("/v1/vote_results", "/{gp}/{ityp}/{inr}"),
    ("/v1/vote_results", ID),
    ("/v1/vote_results", LATEST),
    ("/v1/vote_results", LIVE),
    ("/v1/vote_results", SEARCH),
    ("/v1/decrees", "/latest"),
    ("/v1/decrees", "/search"),
    ("/v1/delegates/gov_officials", "/all_at_date"),
    ("/v1/delegates/gov_officials", "/extend/{id}"),
    ("/v1/delegates", "/id/{id}"),
    ("/v1/delegates", "/search"),
    ("/v1/events", "/delete"),
    ("/v1/gov_proposals", "/latest"),
    ("/v1/gov_proposals", "/search"),
    ("/v1/gov_proposals/{gp}/{inr}/mood", "/"),
    ("/v1/statistics", "/delegates_by_call_to_orders"),
    ("/v1/user", "/"),
    ("/v1/decrees", "/latest"),
    ("/v1/decrees", "/search"),
    ("/v1/delegates/gov_officials", "/all_at_date"),
    ("/v1/delegates/gov_officials", "/extend/{id}"),
    ("/v1/delegates", "/id/{id}"),
    ("/v1/delegates", "/search"),
    ("/v1/events", "/delete"),
    ("/v1/gov_proposals", "/latest"),
    ("/v1/gov_proposals", "/search"),
    ("/v1/gov_proposals/{gp}/{inr}/mood", "/"),
    ("/v1/statistics", "/delegates_by_call_to_orders"),
    ("/v1/user", "/"),
];

const GLOBAL_ROUTES: &[(&str, &str)] = &[
    ("", "/add_quiz"),
    ("", "/quizzes"),
    ("", "/walo_questions"),
    ("", ADD_QUIZ),
    ("", QUIZZES),
    ("", WALO_QUESTIONS),
];

const UNDOCUMENTED: &[&str] = &[
    "/api/at/ai_chat_ws",
    "/api/eu/ai_chat_ws",
    "/api/oauth/{provider}",
    "/api/oauth/{provider}/callback",
    "/api/quiz_room",
];

#[test]
fn every_documented_route_matches_its_constant() {
    let openapi = openapi_document();
    for (prefix, path) in PARLIAMENT_ROUTES {
        for parliament in ["at", "eu"] {
            let expected = format!("/api/{parliament}{prefix}{path}");
            let expected = expected.trim_end_matches('/');
            assert!(
                openapi.paths.paths.contains_key(expected),
                "{expected} is not documented by any #[utoipa::path]"
            );
        }
    }
    for (prefix, path) in GLOBAL_ROUTES {
        let expected = format!("/api{prefix}{path}")
            .trim_end_matches('/')
            .to_string();
        assert!(
            openapi.paths.paths.contains_key(&expected),
            "{expected} is not documented by any #[utoipa::path]"
        );
    }
}

#[test]
fn undocumented_routes_stay_out_of_the_document() {
    let openapi = openapi_document();
    for path in UNDOCUMENTED {
        assert!(
            !openapi.paths.paths.contains_key(*path),
            "{path} must not be part of the OpenApi document"
        );
    }
}

#[test]
fn both_parliament_mounts_are_mirrored() {
    let openapi = openapi_document();
    let strip = |prefix: &str| {
        openapi
            .paths
            .paths
            .keys()
            .filter_map(|path| path.strip_prefix(prefix).map(str::to_string))
            .collect::<HashSet<String>>()
    };
    assert_eq!(
        strip("/api/at/"),
        strip("/api/eu/"),
        "the at and the eu mount document different routes"
    );
}

#[test]
fn operation_ids_are_unique() {
    let openapi = openapi_document();
    let mut ids = HashSet::new();
    for (path, item) in openapi.paths.paths.iter() {
        for operation in [
            &item.get,
            &item.put,
            &item.post,
            &item.delete,
            &item.options,
            &item.head,
            &item.patch,
            &item.trace,
        ]
        .into_iter()
        .flatten()
        {
            let id = operation
                .operation_id
                .clone()
                .unwrap_or_else(|| path.clone());
            assert!(ids.insert(id.clone()), "duplicate operationId {id}");
        }
    }
    assert!(ids.len() > 100, "only {} operations documented", ids.len());
}

#[test]
fn every_documented_path_is_listed() {
    let openapi = openapi_document();
    let mut expected: HashSet<String> = HashSet::new();
    for (prefix, path) in PARLIAMENT_ROUTES {
        for parliament in ["at", "eu"] {
            expected.insert(
                format!("/api/{parliament}{prefix}{path}")
                    .trim_end_matches('/')
                    .to_string(),
            );
        }
    }
    for (prefix, path) in GLOBAL_ROUTES {
        expected.insert(
            format!("/api{prefix}{path}")
                .trim_end_matches('/')
                .to_string(),
        );
    }

    let documented: HashSet<String> = openapi.paths.paths.keys().cloned().collect();
    let missing: Vec<&String> = documented.difference(&expected).collect();
    let absent: Vec<&String> = expected.difference(&documented).collect();
    assert!(
        missing.is_empty() && absent.is_empty(),
        "{missing:?} are documented but not listed, {absent:?} are listed but not documented"
    );
}
/// utoipa collects a schema only when it is reachable from a request or response body,
/// so a schema that is merely used as a query parameter has to be registered in
/// [`somes_api::openapi::ApiDoc`] as well. A missing registration leaves every tool
/// reading the document with a `$ref` it cannot resolve.
#[test]
fn every_schema_reference_resolves() {
    let document = serde_json::to_value(openapi_document()).unwrap();
    let schemas = document["components"]["schemas"]
        .as_object()
        .expect("the document has no components")
        .clone();

    let mut unresolved = Vec::new();
    let mut stack = vec![document];
    while let Some(value) = stack.pop() {
        match value {
            serde_json::Value::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), &value) {
                        ("$ref", serde_json::Value::String(reference)) => {
                            if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                                if !schemas.contains_key(name) {
                                    unresolved.push(name.to_string());
                                }
                            }
                        }
                        _ => stack.push(value),
                    }
                }
            }
            serde_json::Value::Array(items) => stack.extend(items),
            _ => {}
        }
    }

    assert!(
        unresolved.is_empty(),
        "schemas referenced but never registered: {unresolved:?}"
    );
}

/// The `Qs<_>` extractor parameters of the search endpoints are plain query parameters,
/// so the fields of the meilisearch filter structs have to show up in the document.
#[test]
fn meilisearch_filters_are_documented_as_query_parameters() {
    let openapi = openapi_document();
    let operation = openapi
        .paths
        .paths
        .get("/api/at/v1/delegates/search")
        .and_then(|item| item.get.as_ref())
        .expect("/api/at/v1/delegates/search is not documented");

    let names: Vec<&str> = operation
        .parameters
        .iter()
        .flatten()
        .map(|parameter| parameter.name.as_str())
        .collect();

    for filter in ["id", "name", "party", "filters"] {
        assert!(
            names.contains(&filter),
            "filter parameter {filter} is not documented: {names:?}"
        );
    }
}
