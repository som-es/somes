use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{bind_values, build_filter, IntoFilterArgument, Manual},
    PgPoolConnection,
};

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct AbsenceFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AbsenceBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    total_absences: i64,
    total_sessions: i64,
    normalized_absences: f64,
    legislative_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AbsenceForDelegate {
    delegate_name: String,
    delegate_party: String,
    total_absences: i64,
    total_sessions: i64,
    normalized_absences: f64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AbsenceByCategory {
    category: String,
    total_absences: i64,
    total_sessions: i64,
    normalized_absences: f64,
}

pub struct AbsenceService;

impl AbsenceService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("ds.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH legislative_period_dates AS (
            SELECT 
                legislative_period, 
                MIN(created_at) AS start_date, 
                MAX(created_at) AS end_date
            FROM 
                plenar_infos
            GROUP BY 
                legislative_period
        ),
        session_counts AS (
            SELECT 
                pf.legislative_period, 
                COUNT(DISTINCT pf.id) AS total_sessions
            FROM 
                plenar_infos pf
            JOIN 
                absences ab ON ab.plenary_session_id = pf.id
            GROUP BY 
                pf.legislative_period
        )
        SELECT DISTINCT ON (d.id)
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            d.gender AS delegate_gender,
            COUNT(DISTINCT ab.id) AS total_absences,
            sc.total_sessions,
            COUNT(DISTINCT ab.id)::FLOAT / NULLIF(sc.total_sessions, 0)::FLOAT AS normalized_absences,
            pf.legislative_period
        FROM
            absences ab
        JOIN 
            delegates d ON ab.delegate_id = d.id
        JOIN 
            mandates m ON m.delegate_id = d.id
        JOIN 
            plenar_infos pf ON pf.id = ab.plenary_session_id
        JOIN 
            legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
        JOIN 
            session_counts sc ON sc.legislative_period = lp.legislative_period 
        WHERE
            {filter_str}
            AND m.start_date <= lp.end_date
            AND (m.end_date IS NULL OR m.end_date >= lp.start_date)
        GROUP BY 
            d.id, d.name, d.gender, m.party, sc.total_sessions, pf.legislative_period
        ORDER BY 
            d.id, total_absences DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, AbsenceBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| {
                println!("Error absences: {:?}", e);
                StatisticsResponse::DbSelectFailure(Some(e))
            })
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut results: Vec<AbsenceForDelegate> = base_data
            .into_iter()
            .map(|item| AbsenceForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                total_absences: item.total_absences,
                total_sessions: item.total_sessions,
                normalized_absences: item.normalized_absences,
            })
            .collect();

        // Sort in Rust based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                a.normalized_absences
                    .partial_cmp(&b.normalized_absences)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| a.total_absences.cmp(&b.total_absences));
        }

        if filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut party_map: std::collections::HashMap<String, (i64, i64, f64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = party_map.entry(item.delegate_party.clone()).or_insert((0, 0, 0.0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
            entry.2 += item.normalized_absences;
        }

        let mut results: Vec<AbsenceByCategory> = party_map
            .into_iter()
            .map(|(party, (total_absences, total_sessions, normalized))| AbsenceByCategory {
                category: party,
                total_absences,
                total_sessions,
                normalized_absences: normalized,
            })
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.normalized_absences
                    .partial_cmp(&a.normalized_absences)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| b.total_absences.cmp(&a.total_absences));
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut gender_map: std::collections::HashMap<String, (i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = gender_map.entry(item.delegate_gender.clone()).or_insert((0, 0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
        }

        let mut results: Vec<AbsenceByCategory> = gender_map
            .into_iter()
            .map(|(gender, (total_absences, total_sessions))| {
                let normalized_absences = if total_sessions > 0 {
                    total_absences as f64 / total_sessions as f64
                } else {
                    0.0
                };
                AbsenceByCategory {
                    category: gender,
                    total_absences,
                    total_sessions,
                    normalized_absences,
                }
            })
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.normalized_absences
                    .partial_cmp(&a.normalized_absences)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| b.total_absences.cmp(&a.total_absences));
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        // Gruppiere nach legislative_period
        let mut period_map: std::collections::HashMap<String, (i64, i64, Vec<f64>)> = std::collections::HashMap::new();
        
        for item in base_data {
            let period = item.legislative_period.unwrap_or("Unknown".to_string());
            let entry = period_map.entry(period)
                .or_insert((0, 0, Vec::new()));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
            entry.2.push(item.normalized_absences);
        }
        
        let mut results: Vec<AbsenceByCategory> = period_map
            .into_iter()
            .map(|(period, (total_absences, total_sessions, normalized_values))| {
                let avg_normalized = if !normalized_values.is_empty() {
                    normalized_values.iter().sum::<f64>() / normalized_values.len() as f64
                } else {
                    0.0
                };
                
                AbsenceByCategory {
                    category: period,
                    total_absences,
                    total_sessions,
                    normalized_absences: avg_normalized,
                }
            })
            .collect();
        
        if !filter.is_desc {
            results.reverse();
        }
        
        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        // For age grouping, we'd need to calculate ages from birth dates
        // This is a simplified version - you might need to adjust based on your actual age calculation logic
        let mut results: Vec<AbsenceByCategory> = vec![
            AbsenceByCategory {
                category: "18-30".to_string(),
                total_absences: 0,
                total_sessions: 0,
                normalized_absences: 0.0,
            },
            AbsenceByCategory {
                category: "31-40".to_string(),
                total_absences: 0,
                total_sessions: 0,
                normalized_absences: 0.0,
            },
            AbsenceByCategory {
                category: "41-50".to_string(),
                total_absences: 0,
                total_sessions: 0,
                normalized_absences: 0.0,
            },
            AbsenceByCategory {
                category: "51-60".to_string(),
                total_absences: 0,
                total_sessions: 0,
                normalized_absences: 0.0,
            },
            AbsenceByCategory {
                category: "60+".to_string(),
                total_absences: 0,
                total_sessions: 0,
                normalized_absences: 0.0,
            },
        ];

        // Note: You'll need to implement actual age calculation logic here
        // This is a placeholder that aggregates all data into "Unknown" category
        let total_absences: i64 = base_data.iter().map(|item| item.total_absences).sum();
        let total_sessions: i64 = base_data.iter().map(|item| item.total_sessions).sum();
        let total_normalized: f64 = base_data.iter().map(|item| item.normalized_absences).sum();

        results.push(AbsenceByCategory {
            category: "Unknown".to_string(),
            total_absences,
            total_sessions,
            normalized_absences: total_normalized,
        });

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.normalized_absences
                    .partial_cmp(&a.normalized_absences)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| b.total_absences.cmp(&a.total_absences));
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Legacy endpoint functions for backward compatibility
pub async fn absences_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AbsenceFilter>>,
) -> Result<Json<Vec<AbsenceForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AbsenceService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn absences_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AbsenceFilter>>,
) -> Result<Json<Vec<AbsenceByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AbsenceService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn absences_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AbsenceFilter>>,
) -> Result<Json<Vec<AbsenceByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AbsenceService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn absences_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AbsenceFilter>>,
) -> Result<Json<Vec<AbsenceByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AbsenceService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn absences_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AbsenceFilter>>,
) -> Result<Json<Vec<AbsenceByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AbsenceService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}
