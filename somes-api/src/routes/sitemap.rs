use axum::{Json, extract::Query};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState, PgPoolConnection, RedisConnection, get_json_cache, routes::FilterError,
    set_json_cache,
};

const DEFAULT_ENTRIES_PER_PAGE: i64 = 1000;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapEntries<T> {
    pub entries: Vec<T>,
    pub entry_count: i64,
    pub max_page: i64,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapVoteResultPath {
    pub gp: String,
    pub ityp: String,
    pub inr: i32,
    pub lastmod: DateTime<Utc>,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapGovProposalPath {
    pub gp: String,
    pub inr: i32,
    pub lastmod: DateTime<Utc>,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapDecreePath {
    pub ris_id: String,
    pub lastmod: DateTime<Utc>,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapQuestionPath {
    pub id: i64,
    pub lastmod: DateTime<Utc>,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapGpCount {
    pub gp: String,
    pub entry_count: i64,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct SitemapSummary {
    pub decrees: i64,
    pub gov_proposals: i64,
    pub questions: i64,
    pub vote_results: Vec<SitemapGpCount>,
}

#[derive(ToSchema, IntoParams, Debug, Clone, Deserialize)]
pub struct SitemapQuery {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub entries_per_page: Option<i64>,
    #[serde(default)]
    pub gp: Option<String>,
}

impl SitemapQuery {
    fn entries_per_page(&self) -> i64 {
        self.entries_per_page
            .unwrap_or(DEFAULT_ENTRIES_PER_PAGE)
            .clamp(1, DEFAULT_ENTRIES_PER_PAGE)
    }

    fn offset(&self) -> i64 {
        (self.page.unwrap_or(1).max(1) - 1) * self.entries_per_page()
    }
}

pub fn create_sitemap_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(sitemap_summary_route))
        .routes(routes!(sitemap_vote_results_route))
        .routes(routes!(sitemap_gov_proposals_route))
        .routes(routes!(sitemap_decrees_route))
        .routes(routes!(sitemap_questions_route))
}

fn max_page(entries: i64, entries_per_page: i64) -> i64 {
    entries.div_euclid(entries_per_page) + i64::from(entries.rem_euclid(entries_per_page) != 0)
}

#[utoipa::path(
    get,
    path = "/summary",
    tag = "sitemap",
    responses(
        (status = 200, description = "Sitemap summary", body = SitemapSummary),
    )
)]
pub async fn sitemap_summary_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<SitemapSummary>, FilterError> {
    let key = "sitemap/summary";
    if let Some(summary) = get_json_cache::<SitemapSummary>(&mut redis_con, key).await {
        return Ok(Json(summary));
    }

    let summary = summary_sqlx(&pg).await?;
    set_json_cache(&mut redis_con, key, &summary).await;
    Ok(Json(summary))
}

#[utoipa::path(
    get,
    path = "/vote_results",
    tag = "sitemap",
    params(SitemapQuery),
    responses(
        (status = 200, description = "Sitemap vote results", body = SitemapEntries<SitemapVoteResultPath>),
    )
)]
pub async fn sitemap_vote_results_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<SitemapQuery>,
) -> Result<Json<SitemapEntries<SitemapVoteResultPath>>, FilterError> {
    let entries_per_page = query.entries_per_page();
    let key = format!(
        "sitemap/vote_results/{}/{}",
        query.gp.as_deref().unwrap_or("*"),
        query.offset()
    );

    if let Some(entries) =
        get_json_cache::<SitemapEntries<SitemapVoteResultPath>>(&mut redis_con, &key).await
    {
        return Ok(Json(entries));
    }

    let rows = sqlx::query!(
        r#"
        select gp, ityp, inr,
            greatest(
                updated_at,
                coalesce(raw_data_updated_at, 'epoch'::timestamptz),
                coalesce(vote_date::timestamptz, 'epoch'::timestamptz)
            ) as "lastmod!",
            count(*) over () as "entry_count!"
        from legislative_initiatives
        where is_voteable_on and ($1::varchar is null or gp = $1)
        order by id
        limit $2 offset $3
        "#,
        query.gp,
        entries_per_page,
        query.offset()
    )
    .fetch_all(&pg)
    .await?;

    let entry_count = rows.first().map(|row| row.entry_count).unwrap_or(0);
    let entries = SitemapEntries {
        entries: rows
            .into_iter()
            .map(|row| SitemapVoteResultPath {
                gp: row.gp,
                ityp: row.ityp,
                inr: row.inr,
                lastmod: row.lastmod,
            })
            .collect(),
        entry_count,
        max_page: max_page(entry_count, entries_per_page),
    };

    set_json_cache(&mut redis_con, &key, &entries).await;
    Ok(Json(entries))
}

#[utoipa::path(
    get,
    path = "/gov_proposals",
    tag = "sitemap",
    params(SitemapQuery),
    responses(
        (status = 200, description = "Sitemap gov proposals", body = SitemapEntries<SitemapGovProposalPath>),
    )
)]
pub async fn sitemap_gov_proposals_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<SitemapQuery>,
) -> Result<Json<SitemapEntries<SitemapGovProposalPath>>, FilterError> {
    let entries_per_page = query.entries_per_page();
    let key = format!("sitemap/gov_proposals/{}", query.offset());

    if let Some(entries) =
        get_json_cache::<SitemapEntries<SitemapGovProposalPath>>(&mut redis_con, &key).await
    {
        return Ok(Json(entries));
    }

    let rows = sqlx::query!(
        r#"
        select gp, inr,
            greatest(
                raw_data_created_at,
                coalesce(raw_data_updated_at, 'epoch'::timestamptz),
                coalesce(updated_at, 'epoch'::timestamptz)
            ) as "lastmod!",
            count(*) over () as "entry_count!"
        from ministrial_proposals
        order by id
        limit $1 offset $2
        "#,
        entries_per_page,
        query.offset()
    )
    .fetch_all(&pg)
    .await?;

    let entry_count = rows.first().map(|row| row.entry_count).unwrap_or(0);
    let entries = SitemapEntries {
        entries: rows
            .into_iter()
            .map(|row| SitemapGovProposalPath {
                gp: row.gp,
                inr: row.inr,
                lastmod: row.lastmod,
            })
            .collect(),
        entry_count,
        max_page: max_page(entry_count, entries_per_page),
    };

    set_json_cache(&mut redis_con, &key, &entries).await;
    Ok(Json(entries))
}

#[utoipa::path(
    get,
    path = "/decrees",
    tag = "sitemap",
    params(SitemapQuery),
    responses(
        (status = 200, description = "Sitemap decrees", body = SitemapEntries<SitemapDecreePath>),
    )
)]
pub async fn sitemap_decrees_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<SitemapQuery>,
) -> Result<Json<SitemapEntries<SitemapDecreePath>>, FilterError> {
    let entries_per_page = query.entries_per_page();
    let key = format!("sitemap/decrees/{}", query.offset());

    if let Some(entries) =
        get_json_cache::<SitemapEntries<SitemapDecreePath>>(&mut redis_con, &key).await
    {
        return Ok(Json(entries));
    }

    let rows = sqlx::query!(
        r#"
        select ris_id,
            greatest(
                publication_date::timestamptz,
                coalesce(updated_at, 'epoch'::timestamptz)
            ) as "lastmod!",
            count(*) over () as "entry_count!"
        from ministrial_decrees
        order by publication_date desc, ris_id
        limit $1 offset $2
        "#,
        entries_per_page,
        query.offset()
    )
    .fetch_all(&pg)
    .await?;

    let entry_count = rows.first().map(|row| row.entry_count).unwrap_or(0);
    let entries = SitemapEntries {
        entries: rows
            .into_iter()
            .map(|row| SitemapDecreePath {
                ris_id: row.ris_id,
                lastmod: row.lastmod,
            })
            .collect(),
        entry_count,
        max_page: max_page(entry_count, entries_per_page),
    };

    set_json_cache(&mut redis_con, &key, &entries).await;
    Ok(Json(entries))
}

#[utoipa::path(
    get,
    path = "/questions",
    tag = "sitemap",
    params(SitemapQuery),
    responses(
        (status = 200, description = "Sitemap questions", body = SitemapEntries<SitemapQuestionPath>),
    )
)]
pub async fn sitemap_questions_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(query): Query<SitemapQuery>,
) -> Result<Json<SitemapEntries<SitemapQuestionPath>>, FilterError> {
    let entries_per_page = query.entries_per_page();
    let key = format!("sitemap/questions/{}", query.offset());

    if let Some(entries) =
        get_json_cache::<SitemapEntries<SitemapQuestionPath>>(&mut redis_con, &key).await
    {
        return Ok(Json(entries));
    }

    let rows = sqlx::query!(
        r#"
        select q.id,
            greatest(
                q.updated_at,
                coalesce(max(a.received_at), 'epoch'::timestamptz)
            ) as "lastmod!",
            count(*) over () as "entry_count!"
        from delegate_questions q
        left join delegate_question_answers a on a.question_id = q.id
        where q.status in ('sent', 'answered')
        group by q.id, q.updated_at
        order by q.id
        limit $1 offset $2
        "#,
        entries_per_page,
        query.offset()
    )
    .fetch_all(&pg)
    .await?;

    let entry_count = rows.first().map(|row| row.entry_count).unwrap_or(0);
    let entries = SitemapEntries {
        entries: rows
            .into_iter()
            .map(|row| SitemapQuestionPath {
                id: row.id,
                lastmod: row.lastmod,
            })
            .collect(),
        entry_count,
        max_page: max_page(entry_count, entries_per_page),
    };

    set_json_cache(&mut redis_con, &key, &entries).await;
    Ok(Json(entries))
}

async fn summary_sqlx(pg: &PgPool) -> Result<SitemapSummary, FilterError> {
    let counts = sqlx::query!(
        r#"
        select
            (select count(*) from ministrial_decrees) as "decrees!",
            (select count(*) from ministrial_proposals) as "gov_proposals!",
            (select count(*) from delegate_questions where status in ('sent', 'answered'))
                as "questions!"
        "#,
    )
    .fetch_one(pg)
    .await?;

    let gps = sqlx::query!(
        r#"
        select gp, count(*) as "entry_count!"
        from legislative_initiatives
        where accepted is not null
        group by gp
        order by gp
        "#,
    )
    .fetch_all(pg)
    .await?;

    Ok(SitemapSummary {
        decrees: counts.decrees,
        gov_proposals: counts.gov_proposals,
        questions: counts.questions,
        vote_results: gps
            .into_iter()
            .map(|row| SitemapGpCount {
                gp: row.gp,
                entry_count: row.entry_count,
            })
            .collect(),
    })
}
