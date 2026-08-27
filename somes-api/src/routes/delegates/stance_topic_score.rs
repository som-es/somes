use std::collections::HashMap;

use combx::with_data::unique_topics::{TopicsMapper, translate_topics_with_eurovoc};
use common_scrapes::language::Language;
use reqwest::StatusCode;
use somes_common_lib::{StanceTopicInfluences, StanceTopicScore};
use sqlx::{PgPool, query};

use crate::{GenericError::Custom, routes::DelegateError};

pub async fn extract_stance_topic_score_by_delegate(
    pg: &PgPool,
    delegate_id: i32,
    topics_mapper: &TopicsMapper,
    language: Language,
) -> Result<(Vec<StanceTopicInfluences>, Vec<StanceTopicScore>), DelegateError> {
    let mut stance_scores = query!(
        "select DISTINCT ON (pa.question_id)
            answer, question, stance_llm, stance, pro_strong_ref_score, contra_strong_ref_score, ref_score, COALESCE(lis.influences, '{}') AS influences, COALESCE(lis.topics, '{}') AS topics
        from
            political_opinions po
        left join
            (select question_id, ARRAY_AGG(topic) as topics, ARRAY_AGG(influence) as influences from political_questions_topics_influence lq group by question_id) as lis
        on lis.question_id = po.question_id
        join political_answers pa on pa.question_id = po.question_id and pa.delegate_id = po.delegate_id and pa.id = po.answer_id
        inner join political_questions pq on pq.id = pa.question_id
        where po.delegate_id = $1
        order by pa.question_id, created_at DESC;
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await.map_err(|e| DelegateError::SqlFailure(e))?;

    let mut topics_scores = HashMap::<String, (f64, usize)>::new();

    let german_topics = topics_mapper
        .unique_topics
        .lang_to_topics
        .get(&Language::De)
        .ok_or(DelegateError::GenericError(Custom((
            StatusCode::INTERNAL_SERVER_ERROR,
            "german eurovoc topics are not available",
        ))))?;

    for stance_score in &mut stance_scores {
        let Some(topics) = &mut stance_score.topics else {
            continue;
        };

        translate_topics_with_eurovoc(
            &topics_mapper.unique_topics,
            language,
            german_topics,
            topics,
        );
    }

    let stance_scores = stance_scores
        .into_iter()
        .flat_map(|stance_score| {
            let topic_influences = stance_score
                .topics
                .unwrap_or_default()
                .iter()
                .zip(&stance_score.influences.unwrap_or_default())
                .map(|(topic, influence)| {
                    let default = if stance_score.stance_llm.to_lowercase().contains("positive") {
                        *influence * stance_score.ref_score.abs()
                    } else if stance_score.stance_llm.to_lowercase().contains("negative") {
                        *influence * stance_score.ref_score.abs() * -1.
                    } else {
                        0.
                    };

                    StanceTopicScore {
                        topic: topic.into(),
                        score: default,
                        broken_down_score: Default::default(),
                    }
                })
                .collect::<Vec<_>>();
            Some(StanceTopicInfluences {
                question: stance_score.question,
                answer: stance_score.answer,
                stance_llm: stance_score.stance_llm,
                topic_influences,
            })
        })
        .collect::<Vec<_>>();

    for stance_score in &stance_scores {
        if stance_score.stance_llm.to_lowercase().contains("neutral") {
            continue;
        }
        for topic_influence in &stance_score.topic_influences {
            topics_scores
                .entry(topic_influence.topic.to_string())
                .and_modify(|x| {
                    x.0 += topic_influence.score;
                    x.1 += 1;
                })
                .or_insert((topic_influence.score, 1));
        }
    }

    Ok((
        stance_scores,
        topics_scores
            .into_iter()
            .map(|(topic, score)| {
                let (score, count) = score;
                StanceTopicScore {
                    topic,
                    score: 2.7 * score / count as f64,
                    broken_down_score: Default::default(),
                }
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {

    use combx::{connect_pg, with_data::unique_topics::TopicsMapper};

    use crate::routes::delegates::stance_topic_score::extract_stance_topic_score_by_delegate;

    #[tokio::test]
    async fn test_extract_stance_topic_score_by_delegate() {
        let pg = connect_pg().await;
        let topics_mapper = TopicsMapper::new(&pg).await.unwrap();
        let res = extract_stance_topic_score_by_delegate(
            &pg,
            35520,
            &topics_mapper,
            common_scrapes::language::Language::En,
        )
        .await
        .unwrap();
        for r in res.0 {
            println!("res: {r:?}");
        }
    }
}
