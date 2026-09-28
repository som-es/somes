use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, prelude::FromRow};
use utoipa::ToSchema;

use crate::{
    PgPoolConnection,
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{
        IntoFilterArgument, Manual, bind_values, build_filter,
    },
};

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityBase {
    delegate_id: i32,
    latest_activity_date: Option<chrono::NaiveDate>,
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    delegate_gender: Option<String>,
    complexity_score: f64,
    total_proposals: i64,
    legislative_period: Option<String>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    complexity_score: f64,
    total_proposals: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityByCategory {
    category: String,
    average_complexity: f64,
    total_proposals: i64,
    delegate_count: i64,
}

pub struct ComplexityService;

impl ComplexityService {
    // Merge a person's contributions before averaging people. A person with
    // multiple parties or periods must not receive extra weight in a group.
    fn merge_by_delegate(base_data: Vec<ComplexityBase>) -> Vec<ComplexityBase> {
        let mut delegates: std::collections::HashMap<i32, ComplexityBase> =
            std::collections::HashMap::new();
        for item in base_data {
            let weighted_score = item.complexity_score * item.total_proposals as f64;
            match delegates.entry(item.delegate_id) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    let mut item = item;
                    item.complexity_score = weighted_score;
                    entry.insert(item);
                }
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let current = entry.get_mut();
                    current.complexity_score += weighted_score;
                    current.total_proposals += item.total_proposals;
                    if item.latest_activity_date > current.latest_activity_date {
                        current.delegate_party = item.delegate_party;
                        current.delegate_filter_party = item.delegate_filter_party;
                        current.latest_activity_date = item.latest_activity_date;
                        current.delegate_age_bucket = item.delegate_age_bucket;
                        current.legislative_period = item.legislative_period;
                    }
                }
            }
        }
        delegates
            .into_values()
            .map(|mut item| {
                item.complexity_score = if item.total_proposals > 0 {
                    item.complexity_score / item.total_proposals as f64
                } else {
                    0.0
                };
                item
            })
            .collect()
    }

    fn aggregate_by_category(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
        category: impl Fn(&ComplexityBase) -> Option<String>,
    ) -> Vec<ComplexityByCategory> {
        let mut groups: std::collections::HashMap<String, Vec<ComplexityBase>> =
            std::collections::HashMap::new();
        for item in base_data {
            if let Some(key) = category(&item) {
                groups.entry(key).or_default().push(item);
            }
        }
        let mut results: Vec<_> = groups
            .into_iter()
            .map(|(category, rows)| {
                let people = Self::merge_by_delegate(rows);
                ComplexityByCategory {
                    category,
                    average_complexity: people.iter().map(|p| p.complexity_score).sum::<f64>()
                        / people.len() as f64,
                    total_proposals: people.iter().map(|p| p.total_proposals).sum(),
                    delegate_count: people.len() as i64,
                }
            })
            .collect();
        results.sort_by(|a, b| {
            let order = a.average_complexity.total_cmp(&b.average_complexity);
            (if is_desc { order.reverse() } else { order })
                .then_with(|| a.category.cmp(&b.category))
        });
        results
    }

    fn aggregate_by_party(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        // A party gets only the initiatives attributed to it at the time.
        Self::aggregate_by_category(base_data, is_desc, |item| {
            Some(item.delegate_filter_party.clone())
        })
    }

    fn aggregate_by_gender(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        Self::aggregate_by_category(base_data, is_desc, |item| {
            Some(
                item.delegate_gender
                    .clone()
                    .unwrap_or_else(|| "Unknown".into()),
            )
        })
    }

    fn aggregate_by_legis(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        Self::aggregate_by_category(base_data, is_desc, |item| item.legislative_period.clone())
    }

    fn aggregate_by_age(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        // Use each person's age at their latest initiative within the selection.
        Self::aggregate_by_category(Self::merge_by_delegate(base_data), is_desc, |item| {
            Some(item.delegate_age_bucket.clone())
        })
    }

    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityBase>, StatisticsResponse> {
        let filter_arg1 = filter
            .party
            .with_sql_column("COALESCE(m.party, 'Regierungsmitglied')");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = filter.legis_period.with_sql_column("p.gp");
        let filter_arg4 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg1, filter_arg2, filter_arg3, filter_arg4];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH contributions AS (
            SELECT DISTINCT
                p.id AS proposal_id, p.gp, p.created_at,
                d.id AS delegate_id, d.name AS delegate_name,
                COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
                COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
                d.gender AS delegate_gender, d.birthdate,
                CASE
                    WHEN p.ityp = 'AA' THEN 1.2
                    WHEN p.ityp = 'A' THEN 1.2
                    WHEN p.ityp = 'UEA' THEN 1.15
                    WHEN p.ityp = 'I' THEN 1.3
                    ELSE 1.0
                END AS weight
            FROM proposals p
            JOIN proposal_delegates pd ON p.id = pd.proposal_id
            JOIN delegates d ON pd.delegate_id = d.id
            JOIN LATERAL (
                SELECT m.party, m.is_nr, m.is_gov_official
                FROM mandates m
                WHERE m.delegate_id = d.id
                    AND (m.is_nr OR m.is_gov_official)
                    AND (m.start_date IS NULL OR m.start_date <= p.created_at::date)
                    AND (m.end_date IS NULL OR m.end_date >= p.created_at::date)
                ORDER BY m.is_nr DESC NULLS LAST, m.start_date DESC NULLS LAST, m.id DESC
                LIMIT 1
            ) m ON true
            WHERE pd.is_receiver = false AND {filter_str}
        )
        SELECT
            delegate_id, delegate_name, delegate_party, delegate_filter_party, delegate_gender,
            AVG(weight)::float8 AS complexity_score,
            COUNT(*)::bigint AS total_proposals,
            gp AS legislative_period,
            MAX(created_at)::date AS latest_activity_date,
            CASE
                WHEN birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(MAX(created_at), birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(MAX(created_at), birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(MAX(created_at), birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(MAX(created_at), birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM contributions
        GROUP BY delegate_id, delegate_name, delegate_party, delegate_filter_party, delegate_gender, birthdate, gp;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, ComplexityBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut results: Vec<ComplexityForDelegate> = Self::merge_by_delegate(base_data)
            .into_iter()
            .map(|item| ComplexityForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                delegate_filter_party: item.delegate_filter_party,
                complexity_score: item.complexity_score,
                total_proposals: item.total_proposals,
            })
            .collect();

        results.sort_by(|a, b| {
            b.complexity_score
                .partial_cmp(&a.complexity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_party(base_data, filter.is_desc))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(base_data, filter.is_desc))
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_legis(base_data, filter.is_desc))
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_age(base_data, filter.is_desc))
    }
}

#[utoipa::path(
    post,
    path = "/complexity_per_delegate",
    tag = "statistics",
    request_body(content = ComplexityFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Complexity per delegate", body = [ComplexityForDelegate]),
    )
)]
pub async fn complexity_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/complexity_per_party",
    tag = "statistics",
    request_body(content = ComplexityFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Complexity per party", body = [ComplexityByCategory]),
    )
)]
pub async fn complexity_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/complexity_per_gender",
    tag = "statistics",
    request_body(content = ComplexityFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Complexity per gender", body = [ComplexityByCategory]),
    )
)]
pub async fn complexity_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/complexity_per_legis",
    tag = "statistics",
    request_body(content = ComplexityFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Complexity per legis", body = [ComplexityByCategory]),
    )
)]
pub async fn complexity_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/complexity_at_age",
    tag = "statistics",
    request_body(content = ComplexityFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Complexity at age", body = [ComplexityByCategory]),
    )
)]
pub async fn complexity_at_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

#[cfg(test)]
#[path = "tests/complexity.rs"]
mod tests;
