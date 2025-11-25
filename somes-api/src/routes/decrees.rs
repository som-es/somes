use axum::{extract::Query, Json};
use dataservice::combx::{Decree, OptionalDecree};
use meilisearch_sdk::search::SearchResults;
use serde::{Deserialize, Serialize};
use somes_common_lib::{DecreeByRisId, DecreeFilter, Document, Page};
use utoipa::ToSchema;

use super::LegisInitErrorResponse;
use crate::{
    get_json_cache, meilisearch::MeilisearchClient, set_json_cache, PgPoolConnection,
    RedisConnection, DECREES_PER_PAGE,
};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct DecreesWithMaxPage {
    pub decrees: Vec<OptionalDecree>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn get_decrees_per_page(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(page): Query<Page>,
    Json(decree_filter): Json<Option<DecreeFilter>>,
) -> Result<Json<DecreesWithMaxPage>, LegisInitErrorResponse> {
    if page.page < 0 {
        return Err(LegisInitErrorResponse::InvalidPage);
    }

    let key = format!("decrees_per_page/{page:?}/{decree_filter:?}");
    if let Some(decrees_with_max_page) =
        get_json_cache::<DecreesWithMaxPage>(&mut redis_con, &key).await
    {
        return Ok(Json(decrees_with_max_page));
    }
    let decrees_per_page = get_decrees_per_page_sqlx(&pg, &page, decree_filter.as_ref()).await?;
    set_json_cache(&mut redis_con, &key, &decrees_per_page).await;
    Ok(Json(decrees_per_page))
}

pub async fn decree_by_ris_id(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(decree_by_ris_id): Query<DecreeByRisId>,
) -> Result<Json<Option<OptionalDecree>>, LegisInitErrorResponse> {
    let key = format!("decree/{}", &decree_by_ris_id.ris_id);
    if let Some(decree) = get_json_cache::<OptionalDecree>(&mut redis_con, &key).await {
        return Ok(Json(Some(decree)));
    }
    let decree = decree_by_ris_id_sqlx(&pg, &decree_by_ris_id.ris_id).await?;
    set_json_cache(&mut redis_con, &key, &decree).await;
    Ok(Json(decree))
}

async fn decree_by_ris_id_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    ris_id: &str,
) -> Result<Option<OptionalDecree>, LegisInitErrorResponse> {
    Ok(sqlx::query_as!(
        OptionalDecree,
        r#"
            select * from ministrial_decrees_with_docs
            WHERE
                ris_id = $1
            "#,
        ris_id
    )
    .fetch_optional(pg)
    .await
    .map_err(|x| {
        LegisInitErrorResponse::GenericErrorResponse(crate::GenericErrorResponse::DbSelectFailure(
            Some(x),
        ))
    })?)
}

pub async fn get_all_decrees_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
) -> sqlx::Result<Vec<OptionalDecree>> {
    Ok(sqlx::query_as!(
        OptionalDecree,
        r#"
        select * from ministrial_decrees_with_docs
        "#,
    )
    .fetch_all(pg)
    .await?
    .into_iter()
    .collect())
}

async fn get_decrees_per_page_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    page: &Page,
    decree_filter: Option<&DecreeFilter>,
) -> Result<DecreesWithMaxPage, LegisInitErrorResponse> {
    let page_elements = DECREES_PER_PAGE.parse().unwrap_or(15);
    let mut entry_count = 0;
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
        decree_filter.map(|x| x.legis_period.as_ref()).flatten(),
        decree_filter
            .map(|x| x.gov_officials.as_ref())
            .flatten()
            .map(|x| &**x),
        page.page * page_elements,
        page_elements
    )
    .fetch_all(pg)
    .await
    .map_err(|x| {
        LegisInitErrorResponse::GenericErrorResponse(crate::GenericErrorResponse::DbSelectFailure(
            Some(x),
        ))
    })?
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
        }
    })
    .collect();

    Ok(DecreesWithMaxPage {
        decrees,
        entry_count,
        max_page: (entry_count as f64 / DECREES_PER_PAGE.parse().unwrap_or(15.)).ceil() as i64,
    })
}

pub async fn decrees_by_search(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(decrees_filter): Json<Option<DecreeFilter>>,
) -> Result<Json<DecreesWithMaxPage>, LegisInitErrorResponse> {
    meilisearch_decrees(
        meilisearch_client,
        search_query,
        page,
        decrees_filter.as_ref(),
    )
    .await
}

async fn meilisearch_decrees(
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    legis_init_filter: Option<&DecreeFilter>,
) -> Result<Json<DecreesWithMaxPage>, LegisInitErrorResponse> {
    let stats = meilisearch_client
        .index("decrees")
        .get_stats()
        .await
        .unwrap();
    println!("{:?}", stats);
    let mut meilisearch_filter = String::new();
    if let Some(filter) = legis_init_filter.as_ref() {
        let mut filter_conditions = vec![];

        if let Some(ref legis_period) = filter.legis_period {
            filter_conditions.push(format!("gp = '{}'", legis_period));
        }
        if let Some(ref legis_period) = filter.gov_officials {
            filter_conditions.push({
                let ors = legis_period
                    .iter()
                    .map(|gov_official| format!("gov_official_id = {gov_official}"))
                    .collect::<Vec<_>>()
                    .join(" OR ");
                format!("({ors})")
            });
        }

        meilisearch_filter = filter_conditions.join(" AND ")
    }
    log::info!("decrees meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let results: SearchResults<OptionalDecree> = meilisearch_client
        .index("decrees")
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&["publication_date:desc"])
        .with_query(&search_query.search)
        .with_hits_per_page(DECREES_PER_PAGE.parse().unwrap_or(16))
        .with_page(page.page as usize)
        .execute()
        .await
        .unwrap();

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let decrees = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();

    Ok(Json(DecreesWithMaxPage {
        decrees,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
    }))
}

#[cfg(test)]
mod tests {
    use dataservice::connect_pg;
    use somes_common_lib::{DecreeFilter, Page};

    use crate::routes::decrees::get_decrees_per_page_sqlx;

    #[tokio::test]
    async fn test_get_decrees_per_page_sqlx() {
        let pg = connect_pg().await;
        let data = get_decrees_per_page_sqlx(
            &pg,
            &Page { page: 1 },
            Some(&DecreeFilter {
                legis_period: Some("XXVII".to_string()),
                gov_officials: Some(vec![57488]),
            }),
        )
        .await;
        println!("data: {data:?}");
    }
}

/*

SELECT
    d.ris_id,
    COUNT(*) OVER() AS entry_count,
    COALESCE(
        json_agg(
            json_build_object(
                'title', doc.title,
                'document_type', doc.document_type,
                'document_url', doc.document_url
            )
        ) FILTER (WHERE doc.id IS NOT NULL),
        '[]'
    ) as documents
FROM
    ministrial_decrees d
LEFT JOIN
    ministrial_decrees_documents doc ON d.id = doc.ministrial_decree_id
WHERE
    ($1::TEXT IS NULL OR d.gp = $1)
    AND ($2::INT[] IS NULL OR d.gov_official_id = ANY($2))
    ris_id = 'BGBLA_2025_II_202'
GROUP BY
    d.gov_official_id, d.ris_id, d.ministrial_issuer,
    d.title, d.short_title, d.publication_date, d.part,
    d.emphasis, d.gp;


 */
