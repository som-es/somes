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
pub struct ActivityFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    activity_score: f64,
    total_proposals: i64,
    mandate_duration_days: i64,
    legislative_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityForDelegate {
    delegate_name: String,
    delegate_party: String,
    activity_score: f64,
    total_proposals: i64,
    mandate_duration_days: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityByCategory {
    category: String,
    activity_score: f64,
    total_proposals: i64,
    delegate_count: i64,
}

pub struct ActivityService;

impl ActivityService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityBase>, StatisticsResponse> {
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        SELECT DISTINCT ON (d.id)
            d.name AS delegate_name,
            COALESCE(MAX(m.party), '') AS delegate_party,
            COALESCE(d.gender, '') AS delegate_gender,
            (
                SUM(
                    CASE 
                        WHEN p.ityp = 'J' THEN 1
                        WHEN p.ityp = 'AA' THEN 1.2 * proposal_counts.proposal_count
                        WHEN p.ityp = 'A' THEN 1.2 * proposal_counts.proposal_count
                        WHEN p.ityp = 'UEA' THEN 1.15 * proposal_counts.proposal_count
                        WHEN p.ityp = 'I' THEN 1.3 * proposal_counts.proposal_count
                        ELSE 0
                    END
                ) / NULLIF(COALESCE(mandate_duration.mandate_duration_days, 1), 0)
            )::float8 AS activity_score,
            COUNT(p.id) AS total_proposals,
            COALESCE(mandate_duration.mandate_duration_days, 0) AS mandate_duration_days,
            NULL::text AS legislative_period
        FROM 
            proposals p
        JOIN 
            proposal_delegates pd ON p.id = pd.proposal_id
        JOIN 
            delegates d ON pd.delegate_id = d.id
        LEFT JOIN mandates m ON m.delegate_id = d.id
        -- LEFT JOIN plenar_infos pf ON pf.id = p.plenar_id -- No plenar_id in proposals table
        LEFT JOIN (
            SELECT 
                p.id AS proposal_id,
                COUNT(p.id) AS proposal_count
            FROM 
                proposals p
            JOIN 
                proposal_delegates pd ON p.id = pd.proposal_id
            WHERE 
                pd.is_receiver = false
            GROUP BY 
                p.id
        ) AS proposal_counts ON p.id = proposal_counts.proposal_id
        LEFT JOIN (
            SELECT
                m.delegate_id,
                SUM(
                    CASE
                        WHEN m.end_date IS NULL THEN (CURRENT_DATE - m.start_date)
                        ELSE (m.end_date - m.start_date)
                    END
                ) AS mandate_duration_days
            FROM mandates m
            WHERE
                m.is_nr = true
                AND m.is_gov_official = true
            GROUP BY m.delegate_id
        ) AS mandate_duration ON d.id = mandate_duration.delegate_id
        WHERE 
            pd.is_receiver = false
            AND {filter_str}
        GROUP BY 
            d.id, d.name, mandate_duration.mandate_duration_days
        ORDER BY 
            d.id, activity_score DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, ActivityBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut results: Vec<ActivityForDelegate> = base_data
            .into_iter()
            .map(|item| ActivityForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                activity_score: item.activity_score,
                total_proposals: item.total_proposals,
                mandate_duration_days: item.mandate_duration_days,
            })
            .collect();

        // Sort by activity score
        results.sort_by(|a, b| {
            b.activity_score
                .partial_cmp(&a.activity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut party_map: std::collections::HashMap<String, (f64, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = party_map.entry(item.delegate_party.clone()).or_insert((0.0, 0, 0));
            entry.0 += item.activity_score;
            entry.1 += item.total_proposals;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<ActivityByCategory> = party_map
            .into_iter()
            .map(|(party, (total_score, total_proposals, delegate_count))| ActivityByCategory {
                category: party,
                activity_score: total_score / delegate_count as f64, // average score
                total_proposals,
                delegate_count,
            })
            .collect();

        results.sort_by(|a, b| {
            b.activity_score
                .partial_cmp(&a.activity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut gender_map: std::collections::HashMap<String, (f64, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = gender_map.entry(item.delegate_gender.clone()).or_insert((0.0, 0, 0));
            entry.0 += item.activity_score;
            entry.1 += item.total_proposals;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<ActivityByCategory> = gender_map
            .into_iter()
            .map(|(gender, (total_score, total_proposals, delegate_count))| ActivityByCategory {
                category: gender,
                activity_score: total_score / delegate_count as f64, // average score
                total_proposals,
                delegate_count,
            })
            .collect();

        results.sort_by(|a, b| {
            b.activity_score
                .partial_cmp(&a.activity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut legis_map: std::collections::HashMap<String, (f64, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            if let Some(period) = item.legislative_period {
                let entry = legis_map.entry(period).or_insert((0.0, 0, 0));
                entry.0 += item.activity_score;
                entry.1 += item.total_proposals;
                entry.2 += 1; // delegate count
            }
        }

        let mut results: Vec<ActivityByCategory> = legis_map
            .into_iter()
            .map(|(period, (total_score, total_proposals, delegate_count))| ActivityByCategory {
                category: period,
                activity_score: total_score / delegate_count as f64, // average score
                total_proposals,
                delegate_count,
            })
            .collect();

        results.sort_by(|a, b| {
            b.activity_score
                .partial_cmp(&a.activity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        // For age grouping, we'd need to calculate ages from birth dates
        // This is a simplified version - you might need to adjust based on your actual age calculation logic
        let mut results: Vec<ActivityByCategory> = vec![
            ActivityByCategory {
                category: "18-30".to_string(),
                activity_score: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ActivityByCategory {
                category: "31-40".to_string(),
                activity_score: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ActivityByCategory {
                category: "41-50".to_string(),
                activity_score: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ActivityByCategory {
                category: "51-60".to_string(),
                activity_score: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ActivityByCategory {
                category: "60+".to_string(),
                activity_score: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
        ];

        // Note: You'll need to implement actual age calculation logic here
        // This is a placeholder that aggregates all data into "Unknown" category
        let total_score: f64 = base_data.iter().map(|item| item.activity_score).sum();
        let total_proposals: i64 = base_data.iter().map(|item| item.total_proposals).sum();
        let delegate_count: i64 = base_data.len() as i64;

        results.push(ActivityByCategory {
            category: "Unknown".to_string(),
            activity_score: if delegate_count > 0 { total_score / delegate_count as f64 } else { 0.0 },
            total_proposals,
            delegate_count,
        });

        results.sort_by(|a, b| {
            b.activity_score
                .partial_cmp(&a.activity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Legislative Initiatives endpoint
#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeInitiativeFilter {
    legis_period: Option<String>,
    accepted: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegislativeInitiativeStats {
    total_initiatives: i64,
}

pub async fn legislative_initiatives_without_simple_majority(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegislativeInitiativeFilter>>,
) -> Result<Json<Vec<LegislativeInitiativeStats>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!("🔍 STATISTICS ENDPOINT: legislative_initiatives_without_simple_majority called with filter: {:?}", filter);

    let filter_arg = filter.legis_period.with_sql_column("gp");
    let filter_arg1 = filter.accepted.with_sql_column("accepted");
    let filter_arg2 = Manual("li.requires_simple_majority = false").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let filter_str = build_filter(&filters);

    let query = format!(
        "
        SELECT
            COUNT(*) AS total_initiatives
        FROM 
            legislative_initiatives li
        WHERE 
            {filter_str}
        "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegislativeInitiativeStats>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    let results = filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;
    println!("✅ STATISTICS ENDPOINT: legislative_initiatives_without_simple_majority returning {} results", results.len());
    Ok(results)
}

// Legacy endpoint functions for backward compatibility
pub async fn activity_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ActivityFilter>>,
) -> Result<Json<Vec<ActivityForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ActivityService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn activity_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ActivityFilter>>,
) -> Result<Json<Vec<ActivityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ActivityService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn activity_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ActivityFilter>>,
) -> Result<Json<Vec<ActivityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ActivityService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn activity_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ActivityFilter>>,
) -> Result<Json<Vec<ActivityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ActivityService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn activity_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ActivityFilter>>,
) -> Result<Json<Vec<ActivityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ActivityService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}
