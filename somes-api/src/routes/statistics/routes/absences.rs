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
    delegate_filter_party: String,
    delegate_gender: Option<String>,
    total_absences: i64,
    total_sessions: i64,
    normalized_absences: f64,
    legislative_period: Option<String>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AbsenceForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
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
    fn sort_categories(results: &mut [AbsenceByCategory], is_desc: bool, normalized: bool) {
        if normalized {
            results.sort_by(|a, b| {
                b.normalized_absences
                    .partial_cmp(&a.normalized_absences)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            results.sort_by(|a, b| b.total_absences.cmp(&a.total_absences));
        }

        if !is_desc {
            results.reverse();
        }
    }

    fn by_category(
        category: String,
        total_absences: i64,
        total_sessions: i64,
    ) -> AbsenceByCategory {
        let normalized_absences = if total_sessions > 0 {
            total_absences as f64 / total_sessions as f64
        } else {
            0.0
        };

        AbsenceByCategory {
            category,
            total_absences,
            total_sessions,
            normalized_absences,
        }
    }

    fn aggregate_by_party(
        base_data: Vec<AbsenceBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<AbsenceByCategory> {
        let mut party_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_filter_party.clone())
                .or_insert((0, 0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
        }

        let mut results: Vec<AbsenceByCategory> = party_map
            .into_iter()
            .map(|(party, (total_absences, total_sessions))| {
                Self::by_category(party, total_absences, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_gender(
        base_data: Vec<AbsenceBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<AbsenceByCategory> {
        let mut gender_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = gender_map
                .entry(
                    item.delegate_gender
                        .clone()
                        .unwrap_or_else(|| "Unknown".into()),
                )
                .or_insert((0, 0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
        }

        let mut results: Vec<AbsenceByCategory> = gender_map
            .into_iter()
            .map(|(gender, (total_absences, total_sessions))| {
                Self::by_category(gender, total_absences, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_legis(
        base_data: Vec<AbsenceBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<AbsenceByCategory> {
        let mut period_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let period = item.legislative_period.unwrap_or("Unknown".to_string());
            let entry = period_map.entry(period).or_insert((0, 0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
        }

        let mut results: Vec<AbsenceByCategory> = period_map
            .into_iter()
            .map(|(period, (total_absences, total_sessions))| {
                Self::by_category(period, total_absences, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_age(
        base_data: Vec<AbsenceBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<AbsenceByCategory> {
        let mut age_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map.entry(item.delegate_age_bucket).or_insert((0, 0));
            entry.0 += item.total_absences;
            entry.1 += item.total_sessions;
        }

        let mut results: Vec<AbsenceByCategory> = age_map
            .into_iter()
            .map(|(category, (total_absences, total_sessions))| {
                Self::by_category(category, total_absences, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter
            .party
            .with_sql_column("COALESCE(m.party, 'Regierungsmitglied')");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH legislative_period_dates AS (
            SELECT
                legislative_period,
                MIN(raw_data_created_at) AS start_date,
                MAX(raw_data_created_at) AS end_date
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
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
            d.gender AS delegate_gender,
            COUNT(DISTINCT ab.id) AS total_absences,
            sc.total_sessions,
            COUNT(DISTINCT ab.id)::FLOAT / NULLIF(sc.total_sessions, 0)::FLOAT AS normalized_absences,
            pf.legislative_period,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
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
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        GROUP BY
            d.id, d.name, d.gender, d.birthdate, d.party, m.party, sc.total_sessions, pf.legislative_period, lp.start_date
        ORDER BY
            d.id, total_absences DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, AbsenceBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        struct DelegateAccumulator {
            delegate_party: String,
            delegate_filter_party: String,
            total_absences: i64,
            total_sessions: i64,
            latest_period_rank: String,
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
                        total_absences: 0,
                        total_sessions: 0,
                        latest_period_rank: String::new(),
                    });

            entry.total_absences += item.total_absences;
            entry.total_sessions += item.total_sessions;

            if period_rank >= entry.latest_period_rank.as_str() {
                entry.delegate_party = item.delegate_party;
                entry.delegate_filter_party = item.delegate_filter_party;
                entry.latest_period_rank = period_rank.to_string();
            }
        }

        let mut results: Vec<AbsenceForDelegate> = delegate_map
            .into_iter()
            .map(|(delegate_name, item)| {
                let normalized_absences = if item.total_sessions > 0 {
                    item.total_absences as f64 / item.total_sessions as f64
                } else {
                    0.0
                };

                AbsenceForDelegate {
                    delegate_name,
                    delegate_party: item.delegate_party,
                    delegate_filter_party: item.delegate_filter_party,
                    total_absences: item.total_absences,
                    total_sessions: item.total_sessions,
                    normalized_absences,
                }
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
        Ok(Self::aggregate_by_party(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_legis(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &AbsenceFilter,
    ) -> Result<Vec<AbsenceByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_age(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
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

#[cfg(test)]
#[path = "tests/absences.rs"]
mod tests;
