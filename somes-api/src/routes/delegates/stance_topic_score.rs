use std::collections::HashMap;

use somes_common_lib::{StanceTopicInfluences, StanceTopicScore};
use sqlx::{query, PgPool};

pub async fn extract_stance_topic_score_by_delegate(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<(Vec<StanceTopicInfluences>, Vec<StanceTopicScore>)> {
    let stance_scores = query!(
        "select 
            answer, question, stance_llm, stance, pro_strong_ref_score, contra_strong_ref_score, ref_score, COALESCE(lis.influences, '{}') AS influences, COALESCE(lis.topics, '{}') AS topics 
        from 
            political_opinions po
        left join
            (select question_id, ARRAY_AGG(topic) as topics, ARRAY_AGG(influence) as influences from political_questions_topics_influence lq group by question_id) as lis
        on lis.question_id = po.question_id
        join political_answers pa on pa.question_id = po.question_id and pa.delegate_id = po.delegate_id
        inner join political_questions pq on pq.id = pa.question_id 
        where po.delegate_id = $1 and model_used = 'gpt4o-mini-de-run'
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    let mut topics_scores = HashMap::<String, (f64, usize)>::new();

    let stance_scores = stance_scores.into_iter().map(|stance_score| {
        let topic_influences = stance_score
            .topics
            .unwrap_or_default()
            .iter()
            .zip(&stance_score.influences.unwrap_or_default())
            .map(|(topic, influence)| {
                let default = if stance_score.stance_llm == "positive" {
                    *influence * stance_score.ref_score.abs()
                } else {
                    *influence * stance_score.ref_score.abs() * -1.
                };

                StanceTopicScore {
                    topic: topic.into(),
                    score: default,
                }
            }).collect::<Vec<_>>();
        StanceTopicInfluences {
            question: stance_score.question,
            answer: stance_score.answer,
            stance_llm: stance_score.stance_llm,
            topic_influences,
        }
    }).collect::<Vec<_>>();


    for stance_score in &stance_scores {
        if stance_score.stance_llm.to_lowercase().contains("neutral") {
            continue;
        }
        for topic_influence in &stance_score.topic_influences
        {
            topics_scores
                .entry(topic_influence.topic.to_string())
                .and_modify(|x| {
                    x.0 += topic_influence.score;
                    x.1 += 1;
                })
                .or_insert((topic_influence.score, 1));
        }
    }

    Ok((stance_scores, topics_scores
        .into_iter()
        .map(|(topic, score)| {
            let (score, count) = score;
            StanceTopicScore {
                topic,
                score: 2.7 * score / count as f64,
            }
        })
        .collect()))
}

#[cfg(test)]
mod tests {
    use dataservice::connect_pg;

    use crate::routes::delegates::left_right_topic_score::extract_left_right_topic_score_by_delegate;

    #[tokio::test]
    async fn test_extract_stance_topic_score_by_delegate() {
        let pg = connect_pg().await;
        let res = extract_left_right_topic_score_by_delegate(&pg, 35520)
            .await
            .unwrap();
        for r in res {
            println!("res: {r:?}");
        }
    }
}
