use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{
        bind_values, build_filter, IntoFilterArgument, Manual,
    },
    PgPoolConnection,
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
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    delegate_gender: String,
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
    fn aggregate_by_party(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_filter_party.clone())
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.complexity_score);
            entry.1 += item.total_proposals;
            entry.2 += 1;
        }

        let mut results: Vec<ComplexityByCategory> = party_map
            .into_iter()
            .map(|(party, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                ComplexityByCategory {
                    category: party,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }

        results
    }

    fn aggregate_by_gender(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                gender_map
                    .entry(item.delegate_gender.clone())
                    .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.complexity_score);
            entry.1 += item.total_proposals;
            entry.2 += 1;
        }

        let mut results: Vec<ComplexityByCategory> = gender_map
            .into_iter()
            .map(|(gender, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                ComplexityByCategory {
                    category: gender,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }

        results
    }

    fn aggregate_by_legis(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        let mut period_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            if let Some(period) = item.legislative_period {
                let entry = period_map.entry(period).or_insert((Vec::new(), 0, 0));
                entry.0.push(item.complexity_score);
                entry.1 += item.total_proposals;
                entry.2 += 1;
            }
        }

        let mut results: Vec<ComplexityByCategory> = period_map
            .into_iter()
            .map(|(period, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                ComplexityByCategory {
                    category: period,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }

        results
    }

    fn aggregate_by_age(
        base_data: Vec<ComplexityBase>,
        is_desc: bool,
    ) -> Vec<ComplexityByCategory> {
        let mut age_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map
                .entry(item.delegate_age_bucket)
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.complexity_score);
            entry.1 += item.total_proposals;
            entry.2 += 1;
        }

        let mut results: Vec<ComplexityByCategory> = age_map
            .into_iter()
            .map(|(category, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                ComplexityByCategory {
                    category,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }

        results
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
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
            COALESCE(d.gender, '') AS delegate_gender,
            AVG(
                CASE
                    WHEN p.ityp = 'J' THEN 1.0
                    WHEN p.ityp = 'AA' THEN 1.2
                    WHEN p.ityp = 'A' THEN 1.2
                    WHEN p.ityp = 'UEA' THEN 1.15
                    WHEN p.ityp = 'I' THEN 1.3
                    ELSE 1.0
                END
            )::FLOAT8 AS complexity_score,
            COUNT(p.id) AS total_proposals,
            p.gp AS legislative_period,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(MAX(p.created_at), d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(MAX(p.created_at), d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(MAX(p.created_at), d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(MAX(p.created_at), d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM
            proposals p
        JOIN
            proposal_delegates pd ON p.id = pd.proposal_id
        JOIN
            delegates d ON pd.delegate_id = d.id
        LEFT JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= p.created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= p.created_at::date)
        WHERE
            pd.is_receiver = false
            AND {filter_str}
        GROUP BY
            d.id, d.name, d.gender, d.birthdate, d.party, m.party, p.gp
        ORDER BY
            d.id, complexity_score DESC;
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

        struct DelegateAccumulator {
            delegate_party: String,
            delegate_filter_party: String,
            weighted_complexity: f64,
            total_proposals: i64,
            latest_period_rank: i32,
        }

        let mut delegate_map: std::collections::HashMap<String, DelegateAccumulator> =
            std::collections::HashMap::new();

        for item in base_data {
            let period_rank = super::legislative_period_rank(item.legislative_period.as_deref());
            let entry =
                delegate_map
                    .entry(item.delegate_name)
                    .or_insert_with(|| DelegateAccumulator {
                        delegate_party: item.delegate_party.clone(),
                        delegate_filter_party: item.delegate_filter_party.clone(),
                        weighted_complexity: 0.0,
                        total_proposals: 0,
                        latest_period_rank: i32::MIN,
                    });

            entry.weighted_complexity += item.complexity_score * item.total_proposals as f64;
            entry.total_proposals += item.total_proposals;

            if period_rank >= entry.latest_period_rank {
                entry.delegate_party = item.delegate_party;
                entry.delegate_filter_party = item.delegate_filter_party;
                entry.latest_period_rank = period_rank;
            }
        }

        let mut results: Vec<ComplexityForDelegate> = delegate_map
            .into_iter()
            .map(|(delegate_name, item)| {
                let complexity_score = if item.total_proposals > 0 {
                    item.weighted_complexity / item.total_proposals as f64
                } else {
                    0.0
                };

                ComplexityForDelegate {
                    delegate_name,
                    delegate_party: item.delegate_party,
                    delegate_filter_party: item.delegate_filter_party,
                    complexity_score,
                    total_proposals: item.total_proposals,
                }
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

pub async fn complexity_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

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
