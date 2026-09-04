use std::collections::HashMap;

use combx::{
    Parliament,
    with_data::unique_topics::{TopicsMapper, translate_topics_with_eurovoc},
};
use common_scrapes::language::Language;
use reqwest::StatusCode;
use somes_common_lib::InterestShare;
use sqlx::PgPool;

use crate::{
    GenericError::Custom,
    routes::DelegateError::{self, SqlFailure},
};

#[derive(Debug, Clone)]
pub struct TopicTalkCount {
    pub topic: String,
    pub talk_count: i64,
}

pub async fn ai_eurovoc_topics(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<(Vec<TopicTalkCount>, Vec<TopicTalkCount>)> {
    let absolute_interests = sqlx::query_as!(
        TopicTalkCount,
        r#"with latest_summaries as (
                select distinct on (lis.legis_init_id) lis.legis_init_id, lis.summary_id
                from legislative_initiative_summaries lis
                inner join summaries s on s.id = lis.summary_id
                order by lis.legis_init_id, s.generated_at desc
            )
            select topic as "topic!", count(*) as "talk_count!"
            from plenar_speeches ps
            inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
            inner join delegates on delegates.id = ps.delegate_id
            inner join latest_summaries ls on ls.legis_init_id = psli.legis_init_id
            inner join summaries s on s.id = ls.summary_id
            cross join lateral jsonb_array_elements_text(s.full_summary -> 'topics') as topic
            where delegates.id = $1
            group by topic
            order by topic"#,
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    let total_talk_counts = sqlx::query_as!(
        TopicTalkCount,
        r#"with latest_summaries as (
                select distinct on (lis.legis_init_id) lis.legis_init_id, lis.summary_id
                from legislative_initiative_summaries lis
                inner join summaries s on s.id = lis.summary_id
                order by lis.legis_init_id, s.generated_at desc
            )
            select topic as "topic!", count(*) as "talk_count!"
            from plenar_speeches ps
            inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
            inner join delegates on delegates.id = ps.delegate_id
            inner join latest_summaries ls on ls.legis_init_id = psli.legis_init_id
            inner join summaries s on s.id = ls.summary_id
            cross join lateral jsonb_array_elements_text(s.full_summary -> 'topics') as topic
            where delegates.council = 'ep' and is_active
            group by topic
            order by topic"#,
    )
    .fetch_all(pg)
    .await?;
    Ok((absolute_interests, total_talk_counts))
}

pub async fn extract_detailed_interests_of_delegate(
    delegate_id: i32,
    pg: &PgPool,
    topics_mapper: &TopicsMapper,
    language: Language,
    parliament: Parliament,
) -> Result<Vec<InterestShare>, DelegateError> {
    let (mut absolute_interests, mut total_talk_counts) = match parliament {
        Parliament::At => {
            let absolute_interests = sqlx::query_as!(TopicTalkCount, r#"select
                    topic, COUNT(*) as "talk_count!" from plenar_speeches ps
                        inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
                        inner join eurovoc_topics_legis_init on eurovoc_topics_legis_init.legislative_initiatives_id=psli.legis_init_id
                        inner join delegates on ps.delegate_id = delegates.id
                    where opinion is not null and delegates.id = $1
                        group by topic
                    order by topic;"#, delegate_id).fetch_all(pg).await?;

            let total_talk_counts = sqlx::query_as!(TopicTalkCount, r#"
                    select topic, COUNT(*) as "talk_count!" from plenar_speeches ps
                    inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
                    inner join eurovoc_topics_legis_init on eurovoc_topics_legis_init.legislative_initiatives_id=psli.legis_init_id
                    inner join delegates on ps.delegate_id = delegates.id
                    where opinion is not null and delegates.council = 'nr' and is_active group by topic order by topic;"#).fetch_all(pg).await?;
            (absolute_interests, total_talk_counts)
        }
        Parliament::Eu => ai_eurovoc_topics(pg, delegate_id).await?,
    };

    let mut interest_shares = Vec::with_capacity(total_talk_counts.len());

    let topic_language = match parliament {
        Parliament::At => Language::De,
        Parliament::Eu => Language::En,
    };

    let expected_topics = topics_mapper
        .unique_eurovoc_topics
        .lang_to_topics
        .get(&topic_language)
        .ok_or(DelegateError::GenericError(Custom((
            StatusCode::INTERNAL_SERVER_ERROR,
            "{} eurovoc topics are not available",
        ))))?;

    for interest in &mut absolute_interests {
        let mut topics = vec![interest.topic.clone()];
        translate_topics_with_eurovoc(
            &topics_mapper.unique_eurovoc_topics,
            language,
            expected_topics,
            &mut topics,
        );
        interest.topic = topics[0].to_string();
    }
    for interest in &mut total_talk_counts {
        let mut topics = vec![interest.topic.clone()];
        translate_topics_with_eurovoc(
            &topics_mapper.unique_eurovoc_topics,
            language,
            expected_topics,
            &mut topics,
        );
        interest.topic = topics[0].to_string();
    }

    let talk_count_sum = absolute_interests
        .iter()
        .map(|val| val.talk_count)
        .sum::<i64>();

    for (absolute_interest, absolute_talk_count) in absolute_interests
        .into_iter()
        .zip(total_talk_counts.into_iter())
    {
        let share_on_total =
            absolute_interest.talk_count as f32 / absolute_talk_count.talk_count as f32;
        let share_on_self = absolute_interest.talk_count as f32 / talk_count_sum as f32;

        interest_shares.push(InterestShare {
            topic_id: topics_mapper
                .unique_eurovoc_topics
                .topic_to_id
                .get(&(absolute_interest.topic.clone(), language))
                .unwrap()
                .id
                .clone(),
            topic: absolute_interest.topic,
            occurences: absolute_interest.talk_count as u32,
            total_share: share_on_total,
            self_share: share_on_self,
        });
    }

    Ok(interest_shares)
}

pub async fn extract_interests_of_delegate(
    delegate_id: i32,
    pg: &PgPool,
    topics_mapper: &TopicsMapper,
    language: Language,
    parliament: Parliament,
) -> Result<Vec<InterestShare>, DelegateError> {
    let (mut absolute_interests, mut total_talk_counts) = match parliament {
        Parliament::At => {
            let absolute_interests = sqlx::query_as!(TopicTalkCount, r#"select
                topic, COUNT(*) as "talk_count!" from plenar_speeches ps
                    inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
                    inner join topics_legis_init on topics_legis_init.legislative_initiatives_id=psli.legis_init_id
                    inner join delegates on ps.delegate_id = delegates.id
                where opinion is not null and delegates.id = $1
                    group by topic
                order by topic;"#,
            delegate_id).fetch_all(pg).await.map_err(|e| SqlFailure(e))?;

            let total_talk_counts = sqlx::query_as!(TopicTalkCount, r#"
                select topic, COUNT(*) as "talk_count!" from plenar_speeches ps
                inner join plenar_speech_legis_inits psli on psli.speech_id = ps.id
                inner join topics_legis_init on topics_legis_init.legislative_initiatives_id=psli.legis_init_id
                inner join delegates on ps.delegate_id = delegates.id
                where opinion is not null and delegates.council = 'nr' and is_active group by topic order by topic;"#
            ).fetch_all(pg).await.map_err(|e| SqlFailure(e))?;

            (absolute_interests, total_talk_counts)
        }
        Parliament::Eu => {
            let (absolute_interests, total_talk_counts) =
                ai_eurovoc_topics(pg, delegate_id).await?;
            (
                to_parent_topics(absolute_interests, topics_mapper, Language::En),
                to_parent_topics(total_talk_counts, topics_mapper, Language::En),
            )
        }
    };
    let mut interest_shares = Vec::with_capacity(total_talk_counts.len());

    // let topic_language = match parliament {
    //     Parliament::At => Language::De,
    //     Parliament::Eu => Language::En,
    // };
    let topic_language = Language::De;

    let expected_topics = topics_mapper
        .unique_topics
        .lang_to_topics
        .get(&topic_language)
        .ok_or(DelegateError::GenericError(Custom((
            StatusCode::INTERNAL_SERVER_ERROR,
            "{} eurovoc topics are not available",
        ))))?;

    for interest in &mut absolute_interests {
        let mut topics = vec![interest.topic.clone()];
        translate_topics_with_eurovoc(
            &topics_mapper.unique_topics,
            language,
            expected_topics,
            &mut topics,
        );
        interest.topic = topics[0].to_string();
    }
    for interest in &mut total_talk_counts {
        let mut topics = vec![interest.topic.clone()];
        translate_topics_with_eurovoc(
            &topics_mapper.unique_topics,
            language,
            expected_topics,
            &mut topics,
        );
        interest.topic = topics[0].to_string();
    }

    let talk_count_sum = absolute_interests
        .iter()
        .map(|val| val.talk_count)
        .sum::<i64>();

    for (absolute_interest, absolute_talk_count) in absolute_interests
        .into_iter()
        .zip(total_talk_counts.into_iter())
    {
        let share_on_total =
            absolute_interest.talk_count as f32 / absolute_talk_count.talk_count as f32;
        let share_on_self = absolute_interest.talk_count as f32 / talk_count_sum as f32;

        interest_shares.push(InterestShare {
            topic_id: topics_mapper
                .unique_topics
                .topic_to_id
                .get(&(absolute_interest.topic.clone(), language))
                .unwrap()
                .id
                .clone(),
            topic: absolute_interest.topic,
            occurences: absolute_interest.talk_count as u32,
            total_share: share_on_total,
            self_share: share_on_self,
        });
    }

    let mut grouped_interest_shares = HashMap::<String, InterestShare>::new();

    for interest_share in interest_shares {
        grouped_interest_shares
            .entry(interest_share.topic_id.clone())
            .and_modify(|existing| {
                existing.occurences += interest_share.occurences;
                existing.total_share += interest_share.total_share;
                existing.self_share += interest_share.self_share;
            })
            .or_insert(interest_share);
    }

    Ok(grouped_interest_shares
        .drain()
        .map(|(_key, value)| value)
        .into_iter()
        .collect())
}

fn to_parent_topics(
    total_talk_counts: Vec<TopicTalkCount>,
    topics_mapper: &TopicsMapper,
    language: Language,
) -> Vec<TopicTalkCount> {
    total_talk_counts
        .into_iter()
        .filter_map(|mut interest| {
            let topic_id = &topics_mapper
                .unique_eurovoc_topics
                .topic_to_id
                .get(&(interest.topic.clone(), language))?
                .id;
            interest.topic = combx::topic_map::translate_topic_to_parent(topic_id).into();
            if interest.topic == "Sonstige" {
                None
            } else {
                Some(interest)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_extract_interests_of_delegate() {
        // let pg_pool = PgPoolOptions::new()
        //     .max_connections(200)
        //     .connect(DATASERVICE_URL)
        //     .await
        //     .unwrap();

        // let mut interests = crate::routes::extract_interests_of_delegate(35520, &pg_pool)
        //     .await
        //     .unwrap();
        // interests.sort_by(|a, b| b.self_share.total_cmp(&a.self_share));
        // println!("interests: {interests:?}");
    }
}
