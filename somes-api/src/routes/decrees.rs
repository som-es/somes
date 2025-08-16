use axum::{extract::Query, Json};
use dataservice::combx::{Decree, Document};
use serde::{Deserialize, Serialize};
use somes_common_lib::{DecreeFilter, Page};
use utoipa::ToSchema;

use super::LegisInitErrorResponse;
use crate::{get_json_cache, set_json_cache, PgPoolConnection, RedisConnection, DECREES_PER_PAGE};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct DecreesWithMaxPage {
    pub decrees: Vec<Decree>,
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

async fn get_decrees_per_page_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    page: &Page,
    decree_filter: Option<&DecreeFilter>,
) -> Result<DecreesWithMaxPage, LegisInitErrorResponse> {
    let page_elements = DECREES_PER_PAGE.parse().unwrap_or(15);
    let mut entry_count = 0;
    let decrees = sqlx::query!(
        r#"
        SELECT 
            d.gov_official_id, 
            d.ris_id, 
            d.ministrial_issuer, 
            d.title, 
            d.short_title, 
            d.publication_date, 
            d.part,
            d.emphasis,
            d.gp,
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
        GROUP BY 
            d.gov_official_id, d.ris_id, d.ministrial_issuer, 
            d.title, d.short_title, d.publication_date, d.part, 
            d.emphasis, d.gp
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
        Decree {
            gov_official_id: Some(x.gov_official_id),
            ris_id: x.ris_id,
            ministrial_issuer: x.ministrial_issuer,
            title: x.title,
            short_title: x.short_title,
            publication_date: x.publication_date,
            part: x.part,
            emphasis: x.emphasis,
            gp: x.gp,
            documents: x
                .documents
                .into_iter()
                .flat_map(|x| serde_json::from_value::<Document>(x))
                .collect(),
        }
    })
    .collect();

    Ok(DecreesWithMaxPage {
        decrees,
        entry_count,
        max_page: (entry_count as f64 / DECREES_PER_PAGE.parse().unwrap_or(15.)).ceil() as i64,
    })
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
