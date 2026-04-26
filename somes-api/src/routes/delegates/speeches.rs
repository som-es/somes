use axum::{extract::Query, Json};
use dataservice::db::models::DbSpeechWithLink;
use serde::{Deserialize, Serialize};
use somes_common_lib::DelegateByIdAndPage;
use sqlx::{query, query_as, PgPool};
use utoipa::ToSchema;

use crate::{routes::DelegateError, PgPoolConnection};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct SpeechesWithMaxPage {
    pub speeches: Vec<DbSpeechWithLink>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn extract_delegate_speeches(
    delegate_id: i32,
    page: i64,
    page_elements: i64,
    pg: &PgPool,
) -> sqlx::Result<(i64, Vec<DbSpeechWithLink>)> {
    // these are not plenar speeches !!! only that are related to a vote result
    // let all_speeches_of_delegate = query!("
    //     select COUNT(*) from speeches
    //     INNER JOIN speeches_html_urls ON speeches.id = speeches_html_urls.speech_id inner join legislative_initiatives li on li.id = legislative_initiatives_id where delegate_id = $1
    // ",
    // delegate_id).fetch_one(pg).await?;

    let all_speeches_of_delegate = query!("
        select COUNT(*) from plenar_speeches
        INNER JOIN plenar_speech_links ON plenar_speeches.id = plenar_speech_links.plenar_speech_id where delegate_id = $1
    ",
    delegate_id).fetch_one(pg).await?;

    Ok((
        all_speeches_of_delegate.count.unwrap_or_default(),
        query_as!(
            DbSpeechWithLink,
            r#"
            SELECT
                duration_in_seconds,
                delegate_id,
                about,
                COALESCE(array_agg(li.legis_init_id), ARRAY[]::integer[]) AS "vote_result_ids!",
                CASE
                    WHEN opinion = 'Pro' THEN true
                    WHEN opinion = 'Contra' THEN false
                    ELSE NULL
                END AS infavor,
                opinion,
                document_url
            FROM plenar_speeches
            INNER JOIN plenar_speech_links ON plenar_speeches.id = plenar_speech_links.plenar_speech_id
            INNER JOIN plenar_speech_legis_inits li ON li.speech_id = plenar_speeches.id
            INNER JOIN debates ON debates.id = plenar_speeches.debate_id
            INNER JOIN plenar_infos pi ON pi.id = debates.plenar_id
            WHERE delegate_id = $1
            GROUP BY
                plenar_speeches.id,
                duration_in_seconds,
                delegate_id,
                about,
                opinion,
                document_url,
                raw_data_created_at
            ORDER BY raw_data_created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            delegate_id,
            page * page_elements,
            page_elements
        )
        .fetch_all(pg)
        .await?,
    ))

    /*
        Ok((all_speeches_of_delegate.count.unwrap_or_default(), query_as!(DbSpeechWithLink, "
            select delegate_id, legislative_initiatives_id, infavor, opinion, document_url from speeches
            INNER JOIN speeches_html_urls ON speeches.id = speeches_html_urls.speech_id inner join legislative_initiatives li on li.id = legislative_initiatives_id where delegate_id = $1 order by created_at desc offset $2 limit $3
        ",
        delegate_id, page * page_elements, page_elements).fetch_all(pg).await?))
    */
}

pub async fn speeches_by_delegate_per_page_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id_and_page): Query<DelegateByIdAndPage>,
) -> Result<Json<SpeechesWithMaxPage>, DelegateError> {
    let DelegateByIdAndPage { delegate_id, page } = delegate_by_id_and_page;

    let page_elements = crate::SPEECHES_PER_PAGE.parse().unwrap_or(16);
    Ok(
        extract_delegate_speeches(delegate_id, page, page_elements, &pg)
            .await
            .map(|(all_speeches_count, speeches)| SpeechesWithMaxPage {
                entry_count: all_speeches_count as i64,
                speeches,
                max_page: (all_speeches_count as f64 / page_elements as f64).ceil() as i64,
            })
            .map(Json)?,
    )
}
