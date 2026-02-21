use axum::{routing::get, Router};
use dataservice::combx::{DbAiSummary, Delegate, DelegateFilter, OptionalDecree};
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::{Document, LATEST, SEARCH};
use somes_macro::MeilisearchFilter;
use somes_meilisearch_filter::FilterArgument;
use utoipa::ToSchema;

use crate::routes::delegate_by_id_sqlx;
use crate::server::AppState;
use dataservice::combx::OptionalDecreeFilter;

mod routes;
pub use routes::*;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize, MeilisearchFilter)]
pub struct DecreeDelegate {
    #[filter(make_optional)]
    pub decree: OptionalDecree,
    pub delegate: Option<Delegate>,
}

pub fn create_decrees_router() -> Router<AppState> {
    Router::new()
        .route(SEARCH, get(decrees_by_search_route))
        // .route(LIVE, post(decrees_per_page_route))
        .route(LATEST, get(latest_decrees_route))
        .route("/ris_id/{ris_id}", get(decree_by_ris_id_route))
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct DecreesWithMaxPage {
    pub decrees: Vec<DecreeDelegate>,
    pub entry_count: i64,
    pub max_page: i64,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
/*
pub async fn decrees_per_page_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(decree_filter): Json<Option<DecreeFilter>>,
) -> Result<Json<DecreesWithMaxPage>, FilterError> {
    if page.page < 0 {
        return Err(FilterError::InvalidPage(page.page as u32));
    }

    let key = format!("decrees_per_page/{page:?}/{decree_filter:?}");
    if let Some(decrees_with_max_page) =
        get_json_cache::<DecreesWithMaxPage>(&mut redis_con, &key).await
    {
        return Ok(Json(decrees_with_max_page));
    }
    let decrees_per_page =
        get_decrees_per_page_sqlx(&mut redis_con, &pg, &page, decree_filter).await?;
    set_json_cache(&mut redis_con, &key, &decrees_per_page).await;
    Ok(Json(decrees_per_page))
}*/

pub async fn get_all_decrees_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    mut redis_con: MultiplexedConnection,
) -> sqlx::Result<Vec<DecreeDelegate>> {
    let decrees = sqlx::query_as!(
        OptionalDecree,
        r#"
        select * from ministrial_decrees_with_docs
        "#,
    )
    .fetch_all(pg)
    .await?
    .into_iter()
    .collect::<Vec<_>>();

    let mut decree_delegates = Vec::with_capacity(decrees.len());
    for decree in decrees {
        let delegate = match decree.gov_official_id {
            Some(delegate_id) => delegate_by_id_sqlx(delegate_id, &pg, &mut redis_con)
                .await
                .ok(),
            None => None,
        };

        decree_delegates.push(DecreeDelegate { delegate, decree })
    }
    Ok(decree_delegates)
}

/*async fn get_decrees_per_page_sqlx(
    redis_con: &mut MultiplexedConnection,
    pg: &sqlx::Pool<sqlx::Postgres>,
    page: &Page,
    decree_filter: Option<DecreeFilter>,
) -> Result<DecreesWithMaxPage, FilterError> {
    let updated_at = crate::meilisearch::get_update_time_of_index(
        redis_con,
        &crate::meilisearch::Index::Decrees,
    )
    .await
    .ok()
    .map(|date| date.naive_local());

    let page_elements = DECREES_PER_PAGE.parse().unwrap_or(15);
    let mut entry_count = 0;

    let decree_filter = decree_filter.unwrap_or_default();
    let gov_officals = decree_filter.gov_officials.map(|x| x.to_value());

    let decrees = sqlx::query!(
        r#"
        select *, COUNT(*) OVER() AS entry_count
        from ministrial_decrees_with_docs as d
        WHERE
            ($1::TEXT IS NULL OR d.gp = $1)
            AND ($2::INT[] IS NULL OR d.gov_official_id = ANY($2))
        order by d.publication_date desc
        offset $3 limit $4
        "#,
        decree_filter.legis_period.map(|x| x.to_value()),
        gov_officals.as_deref(),
        page.page * page_elements,
        page_elements
    )
    .fetch_all(pg)
    .await?
    .into_iter()
    .map(|x| {
        entry_count = x.entry_count.unwrap();
        OptionalDecree {
            gov_official_id: x.gov_official_id,
            ris_id: x.ris_id,
            ministrial_issuer: x.ministrial_issuer,
            title: x.title,
            short_title: x.short_title,
            publication_date: x.publication_date,
            part: x.part,
            emphasis: x.emphasis,
            gp: x.gp,
            eli: x.eli,
            document_url: x.document_url,
            documents: x.documents,
            created_at: x.created_at,
            updated_at: x.updated_at,
        }
    })
    .collect();

    Ok(DecreesWithMaxPage {
        decrees,
        entry_count,
        max_page: (entry_count as f64 / DECREES_PER_PAGE.parse().unwrap_or(15.)).ceil() as i64,
        updated_at,
    })
}*/
