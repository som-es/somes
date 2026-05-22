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
pub struct CallToOrderFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CallToOrdersBase {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    delegate_gender: String,
    total_order_calls: i64,
    total_sessions_attended: Option<i64>,
    normalized_calls_to_order: Option<f64>,
    legislative_period: Option<String>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CallToOrdersForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    total_order_calls: i64,
    total_sessions_attended: i64,
    normalized_calls_to_order: f64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CallToOrdersByCategory {
    category: String,
    total_order_calls: i64,
    total_sessions_attended: Option<i64>,
    normalized_calls_to_order: Option<f64>,
}

pub struct CallToOrdersService;

impl CallToOrdersService {
    fn sort_categories(results: &mut [CallToOrdersByCategory], is_desc: bool, normalized: bool) {
        if normalized {
            if is_desc {
                results.sort_by(|a, b| {
                    b.normalized_calls_to_order
                        .partial_cmp(&a.normalized_calls_to_order)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| a.category.cmp(&b.category))
                });
            } else {
                results.sort_by(|a, b| {
                    a.normalized_calls_to_order
                        .partial_cmp(&b.normalized_calls_to_order)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| a.category.cmp(&b.category))
                });
            }
        } else if is_desc {
            results.sort_by(|a, b| {
                b.total_order_calls
                    .cmp(&a.total_order_calls)
                    .then_with(|| a.category.cmp(&b.category))
            });
        } else {
            results.sort_by(|a, b| {
                a.total_order_calls
                    .cmp(&b.total_order_calls)
                    .then_with(|| a.category.cmp(&b.category))
            });
        }
    }

    fn by_category(
        category: String,
        total_calls: i64,
        total_sessions: i64,
    ) -> CallToOrdersByCategory {
        let normalized_calls_to_order = if total_sessions > 0 {
            total_calls as f64 / total_sessions as f64
        } else {
            0.0
        };

        CallToOrdersByCategory {
            category,
            total_order_calls: total_calls,
            total_sessions_attended: Some(total_sessions),
            normalized_calls_to_order: Some(normalized_calls_to_order),
        }
    }

    fn aggregate_by_party(
        base_data: Vec<CallToOrdersBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<CallToOrdersByCategory> {
        let mut party_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_filter_party.clone())
                .or_insert((0, 0));
            entry.0 += item.total_order_calls;
            entry.1 += item.total_sessions_attended.unwrap_or(0);
        }

        let mut results: Vec<CallToOrdersByCategory> = party_map
            .into_iter()
            .map(|(party, (total_calls, total_sessions))| {
                Self::by_category(party, total_calls, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_gender(
        base_data: Vec<CallToOrdersBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<CallToOrdersByCategory> {
        let mut gender_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = gender_map
                .entry(item.delegate_gender.clone())
                .or_insert((0, 0));
            entry.0 += item.total_order_calls;
            entry.1 += item.total_sessions_attended.unwrap_or(0);
        }

        let mut results: Vec<CallToOrdersByCategory> = gender_map
            .into_iter()
            .map(|(gender, (total_calls, total_sessions))| {
                Self::by_category(gender, total_calls, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_legis(
        base_data: Vec<CallToOrdersBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<CallToOrdersByCategory> {
        let mut legis_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            if let Some(period) = item.legislative_period {
                let entry = legis_map.entry(period).or_insert((0, 0));
                entry.0 += item.total_order_calls;
                entry.1 += item.total_sessions_attended.unwrap_or(0);
            }
        }

        let mut results: Vec<CallToOrdersByCategory> = legis_map
            .into_iter()
            .map(|(period, (total_calls, total_sessions))| {
                Self::by_category(period, total_calls, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    fn aggregate_by_age(
        base_data: Vec<CallToOrdersBase>,
        is_desc: bool,
        normalized: bool,
    ) -> Vec<CallToOrdersByCategory> {
        let mut age_map: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map.entry(item.delegate_age_bucket).or_insert((0, 0));
            entry.0 += item.total_order_calls;
            entry.1 += item.total_sessions_attended.unwrap_or(0);
        }

        let mut results: Vec<CallToOrdersByCategory> = age_map
            .into_iter()
            .map(|(category, (total_calls, total_sessions))| {
                Self::by_category(category, total_calls, total_sessions)
            })
            .collect();

        Self::sort_categories(&mut results, is_desc, normalized);
        results
    }

    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.gender.with_sql_column("d.gender");
        let filter_arg2 = filter
            .party
            .with_sql_column("COALESCE(m.party, 'Regierungsmitglied')");
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
                ps.delegate_id,
                COUNT(DISTINCT pf.id) AS total_sessions_attended
            FROM
                plenar_infos pf
            JOIN
                debates db ON db.plenar_id = pf.id
            JOIN
                plenar_speeches ps ON ps.debate_id = db.id
            GROUP BY
                pf.legislative_period, ps.delegate_id
        )
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
            d.gender AS delegate_gender,
            COUNT(cto.id) AS total_order_calls,
            sc.total_sessions_attended,
            COUNT(DISTINCT cto.id)::FLOAT / NULLIF(sc.total_sessions_attended, 0)::FLOAT AS normalized_calls_to_order,
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
            call_to_order cto
        JOIN
            delegates d ON cto.receiver_id = d.id
        JOIN
            plenar_infos pf ON pf.id = cto.plenar_id
        LEFT JOIN
            mandates m ON m.delegate_id = d.id
        JOIN
            legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
        LEFT JOIN
            session_counts sc ON sc.legislative_period = lp.legislative_period
            AND sc.delegate_id = d.id
        WHERE
            {filter_str}
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        GROUP BY
            d.id, d.name, d.gender, d.birthdate, d.party, m.party, sc.total_sessions_attended, pf.legislative_period, lp.start_date
        ORDER BY
            d.id, total_order_calls DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, CallToOrdersBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        struct DelegateAccumulator {
            delegate_party: String,
            delegate_filter_party: String,
            total_order_calls: i64,
            total_sessions_attended: i64,
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
                        total_order_calls: 0,
                        total_sessions_attended: 0,
                        latest_period_rank: String::new(),
                    });

            entry.total_order_calls += item.total_order_calls;
            entry.total_sessions_attended += item.total_sessions_attended.unwrap_or(0);

            if period_rank >= entry.latest_period_rank.as_str() {
                entry.delegate_party = item.delegate_party;
                entry.delegate_filter_party = item.delegate_filter_party;
                entry.latest_period_rank = period_rank.to_string();
            }
        }

        let mut results: Vec<CallToOrdersForDelegate> = delegate_map
            .into_iter()
            .map(|(delegate_name, item)| {
                let normalized_calls_to_order = if item.total_sessions_attended > 0 {
                    item.total_order_calls as f64 / item.total_sessions_attended as f64
                } else {
                    0.0
                };

                CallToOrdersForDelegate {
                    delegate_name,
                    delegate_party: item.delegate_party,
                    delegate_filter_party: item.delegate_filter_party,
                    total_order_calls: item.total_order_calls,
                    total_sessions_attended: item.total_sessions_attended,
                    normalized_calls_to_order,
                }
            })
            .collect();

        // Sort in Rust based on filter parameters
        if filter.normalized {
            results.sort_by(|a, b| {
                a.normalized_calls_to_order
                    .partial_cmp(&b.normalized_calls_to_order)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.delegate_name.cmp(&b.delegate_name))
            });
        } else {
            results.sort_by(|a, b| {
                a.total_order_calls
                    .cmp(&b.total_order_calls)
                    .then_with(|| a.delegate_name.cmp(&b.delegate_name))
            });
        }

        if filter.is_desc {
            results.reverse();
        }
        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_party(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_legis(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &CallToOrderFilter,
    ) -> Result<Vec<CallToOrdersByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_age(
            base_data,
            filter.is_desc,
            filter.normalized,
        ))
    }
}

pub async fn call_to_orders_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = CallToOrdersService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn call_to_orders_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = CallToOrdersService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn call_to_orders_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = CallToOrdersService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn call_to_orders_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = CallToOrdersService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn call_to_orders_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<CallToOrderFilter>>,
) -> Result<Json<Vec<CallToOrdersByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = CallToOrdersService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

#[cfg(test)]
#[path = "tests/call_to_orders.rs"]
mod tests;
