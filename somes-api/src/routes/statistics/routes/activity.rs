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
pub struct ActivityFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    activity_score: f64,
    raw_activity_score: f64,
    total_proposals: i64,
    session_count: i64,
    legislative_period: Option<String>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityForDelegate {
    delegate_name: String,
    delegate_party: String,
    activity_score: f64,
    raw_activity_score: f64,
    total_proposals: i64,
    session_count: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityByCategory {
    category: String,
    activity_score: f64,
    raw_activity_score: f64,
    total_proposals: i64,
    delegate_count: i64,
}

pub struct ActivityService;

impl ActivityService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &ActivityFilter,
    ) -> Result<Vec<ActivityBase>, StatisticsResponse> {
        let query = "
        WITH initiative_rows AS (
            SELECT
                li.id,
                li.ityp,
                li.gp,
                COALESCE(
                    li.nr_plenary_activity_date,
                    li.raw_data_created_at::date,
                    li.created_at::date
                ) AS activity_date
            FROM legislative_initiatives li
            WHERE ($1::text IS NULL OR li.gp = $1)
        ),
        session_counts AS (
            SELECT
                pf.legislative_period AS gp,
                COUNT(*)::bigint AS session_count
            FROM plenar_infos pf
            GROUP BY pf.legislative_period
        )
        SELECT
            d.name AS delegate_name,
            COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(d.gender, '') AS delegate_gender,
            (
                SUM(
                    CASE
                        WHEN ir.ityp = 'J' THEN 0.35
                        WHEN ir.ityp = 'AA' THEN 0.9
                        WHEN ir.ityp = 'A' THEN 1.0
                        WHEN ir.ityp = 'UEA' THEN 0.75
                        WHEN ir.ityp = 'I' THEN 1.25
                        ELSE 0
                    END
                ) / NULLIF(COALESCE(sc.session_count, 1), 0)
            )::float8 AS activity_score,
            SUM(
                CASE
                    WHEN ir.ityp = 'J' THEN 0.35
                    WHEN ir.ityp = 'AA' THEN 0.9
                    WHEN ir.ityp = 'A' THEN 1.0
                    WHEN ir.ityp = 'UEA' THEN 0.75
                    WHEN ir.ityp = 'I' THEN 1.25
                    ELSE 0
                END
            )::float8 AS raw_activity_score,
            COUNT(DISTINCT ir.id)::bigint AS total_proposals,
            COALESCE(sc.session_count, 0) AS session_count,
            ir.gp AS legislative_period,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(MAX(ir.activity_date), d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(MAX(ir.activity_date), d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(MAX(ir.activity_date), d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(MAX(ir.activity_date), d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM initiative_rows ir
        JOIN legis_init_delegates lid ON lid.legis_init_id = ir.id
        JOIN delegates d ON lid.delegate_id = d.id
        LEFT JOIN LATERAL (
            SELECT m.party
            FROM mandates m
            WHERE m.delegate_id = d.id
                AND (m.is_nr OR m.is_gov_official)
                AND m.start_date <= ir.activity_date
                AND (m.end_date IS NULL OR m.end_date >= ir.activity_date)
            ORDER BY m.is_nr DESC, m.start_date DESC
            LIMIT 1
        ) active_mandate ON true
        LEFT JOIN session_counts sc ON sc.gp = ir.gp
        WHERE
            ($2::text IS NULL OR d.gender = $2)
            AND (
                $3::text IS NULL
                OR COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') = $3
            )
        GROUP BY
            d.id,
            d.name,
            d.birthdate,
            COALESCE(active_mandate.party, d.party, 'Regierungsmitglied'),
            COALESCE(d.gender, ''),
            sc.session_count,
            ir.gp
        ORDER BY
            activity_score DESC;
        ";

        let filtered_query = sqlx::query_as::<Postgres, ActivityBase>(query)
            .bind(filter.legis_period.as_deref())
            .bind(filter.gender.as_deref())
            .bind(filter.party.as_deref());

        filtered_query.fetch_all(pg).await.map_err(|e| {
            println!("Error fetching activity data: {}", e);
            StatisticsResponse::DbSelectFailure(Some(e))
        })
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
                raw_activity_score: item.raw_activity_score,
                total_proposals: item.total_proposals,
                session_count: item.session_count,
            })
            .collect();

        // Sort by activity score based on normalized flag
        if filter.normalized {
            results.sort_by(|a, b| {
                b.activity_score
                    .partial_cmp(&a.activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| {
                b.raw_activity_score
                    .partial_cmp(&a.raw_activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

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

        let mut party_map: std::collections::HashMap<String, (f64, f64, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_party.clone())
                .or_insert((0.0, 0.0, 0, 0));
            entry.0 += item.activity_score;
            entry.1 += item.raw_activity_score;
            entry.2 += item.total_proposals;
            entry.3 += 1; // delegate count
        }

        let mut results: Vec<ActivityByCategory> = party_map
            .into_iter()
            .map(
                |(party, (total_norm_score, total_raw_score, total_proposals, delegate_count))| {
                    ActivityByCategory {
                        category: party,
                        activity_score: total_norm_score / delegate_count as f64, // average normalized score
                        raw_activity_score: total_raw_score / delegate_count as f64, // average raw score
                        total_proposals,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.activity_score
                    .partial_cmp(&a.activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| {
                b.raw_activity_score
                    .partial_cmp(&a.raw_activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

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

        let mut gender_map: std::collections::HashMap<String, (f64, f64, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = gender_map
                .entry(item.delegate_gender.clone())
                .or_insert((0.0, 0.0, 0, 0));
            entry.0 += item.activity_score;
            entry.1 += item.raw_activity_score;
            entry.2 += item.total_proposals;
            entry.3 += 1; // delegate count
        }

        let mut results: Vec<ActivityByCategory> = gender_map
            .into_iter()
            .map(
                |(gender, (total_norm_score, total_raw_score, total_proposals, delegate_count))| {
                    ActivityByCategory {
                        category: gender,
                        activity_score: total_norm_score / delegate_count as f64, // average normalized score
                        raw_activity_score: total_raw_score / delegate_count as f64, // average raw score
                        total_proposals,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.activity_score
                    .partial_cmp(&a.activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| {
                b.raw_activity_score
                    .partial_cmp(&a.raw_activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

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
        let mut legis_map: std::collections::HashMap<String, (f64, f64, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            if let Some(period) = item.legislative_period {
                let entry = legis_map.entry(period).or_insert((0.0, 0.0, 0, 0));
                entry.0 += item.activity_score;
                entry.1 += item.raw_activity_score;
                entry.2 += item.total_proposals;
                entry.3 += 1; // delegate count
            }
        }

        let mut results: Vec<ActivityByCategory> = legis_map
            .into_iter()
            .map(
                |(period, (total_norm_score, total_raw_score, total_proposals, delegate_count))| {
                    ActivityByCategory {
                        category: period,
                        activity_score: total_norm_score / delegate_count as f64, // average normalized score
                        raw_activity_score: total_raw_score / delegate_count as f64, // average raw score
                        total_proposals,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.activity_score
                    .partial_cmp(&a.activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| {
                b.raw_activity_score
                    .partial_cmp(&a.raw_activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

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
        let mut age_map: std::collections::HashMap<String, (f64, f64, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map
                .entry(item.delegate_age_bucket)
                .or_insert((0.0, 0.0, 0, 0));
            entry.0 += item.activity_score;
            entry.1 += item.raw_activity_score;
            entry.2 += item.total_proposals;
            entry.3 += 1;
        }

        let mut results: Vec<ActivityByCategory> = age_map
            .into_iter()
            .map(
                |(
                    category,
                    (total_norm_score, total_raw_score, total_proposals, delegate_count),
                )| ActivityByCategory {
                    category,
                    activity_score: total_norm_score / delegate_count as f64,
                    raw_activity_score: total_raw_score / delegate_count as f64,
                    total_proposals,
                    delegate_count,
                },
            )
            .collect();

        // Sort based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                b.activity_score
                    .partial_cmp(&a.activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| {
                b.raw_activity_score
                    .partial_cmp(&a.raw_activity_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

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
