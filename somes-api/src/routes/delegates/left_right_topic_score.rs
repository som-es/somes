use std::collections::HashMap;

use somes_common_lib::StanceTopicScore;
use sqlx::{query, PgPool};

pub async fn extract_left_right_topic_score_by_delegate(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<Vec<StanceTopicScore>> {
    let stance_scores = query!(
        "select 
            question, answer, is_liberal, is_left, stance_llm, stance, pro_strong_ref_score, contra_strong_ref_score, ref_score, COALESCE(lis.topics, '{}') AS topics 
        from 
            political_opinions po
        left join
            (select question_id, ARRAY_AGG(topic) as topics from political_questions_topics lq group by question_id) as lis
        on lis.question_id = po.question_id
        join political_answers pa on pa.question_id = po.question_id and pa.delegate_id = po.delegate_id
        inner join political_questions pq on pq.id = pa.question_id 
        where po.delegate_id = $1 and model_used = 'gpt4o-mini-de-run'
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    let mut topics_scores = HashMap::<String, (f64, f64, usize)>::new();

    for stance_score in stance_scores {
        if stance_score.stance_llm.to_lowercase().contains("neutral") {
            continue;
        }

        let default = if stance_score.is_left.unwrap_or_default()
            || stance_score.is_liberal.unwrap_or_default()
        {
            (
                stance_score.pro_strong_ref_score,
                stance_score.contra_strong_ref_score,
            )
        } else if stance_score.is_left.is_some() || stance_score.is_liberal.is_some() {
            (
                stance_score.contra_strong_ref_score,
                stance_score.pro_strong_ref_score,
            )
        } else {
            continue;
        };

        for topic in &stance_score.topics.unwrap_or_default() {
            topics_scores
                .entry(topic.to_string())
                .and_modify(|x| {
                    x.0 += default.0;
                    x.1 += default.1;
                    x.2 += 1;
                })
                .or_insert((default.0, default.1, 1));
        }
    }

    Ok(topics_scores
        .into_iter()
        .map(|(topic, score)| {
            let (pos_score, contra_score, count) = score;
            StanceTopicScore {
                topic,
                score: -1.8 * (pos_score - contra_score) / count as f64,
            }
        })
        .collect())
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
