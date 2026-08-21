use std::collections::HashMap;

use somes_common_lib::{PoliticalScore, StanceTopicScore};
use sqlx::{PgPool, query};

pub async fn extract_left_right_topic_score_by_delegate(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<Vec<StanceTopicScore>> {
    let stance_scores = query!(
        "select DISTINCT ON (pa.question_id)
            question, answer, is_liberal, is_left, stance_llm, stance, pro_strong_ref_score, contra_strong_ref_score, ref_score, COALESCE(lis.topics, '{}') AS topics
        from
            political_opinions po
        left join
            (select question_id, ARRAY_AGG(topic) as topics from political_questions_topics lq group by question_id) as lis
        on lis.question_id = po.question_id
        join political_answers pa on pa.question_id = po.question_id and pa.delegate_id = po.delegate_id
        inner join political_questions pq on pq.id = pa.question_id
        where po.delegate_id = $1
        order by pa.question_id, created_at DESC
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    let mut topics_scores = HashMap::<String, (PoliticalScore, usize)>::new();

    for stance_score in stance_scores {
        if stance_score.stance_llm.to_lowercase().contains("neutral") {
            continue;
        }

        let mut temp_score = PoliticalScore::default();
        if stance_score.is_left.unwrap_or_default() {
            temp_score.socialist += stance_score.pro_strong_ref_score;
            temp_score.capitalist += stance_score.contra_strong_ref_score;
        } else if stance_score.is_left.is_some() {
            temp_score.capitalist += stance_score.pro_strong_ref_score;
            temp_score.socialist += stance_score.contra_strong_ref_score;
        }

        if stance_score.is_liberal.unwrap_or_default() {
            temp_score.liberal += stance_score.pro_strong_ref_score;
            temp_score.authoritarian += stance_score.contra_strong_ref_score;
        } else if stance_score.is_liberal.is_some() {
            temp_score.authoritarian += stance_score.pro_strong_ref_score;
            temp_score.liberal += stance_score.contra_strong_ref_score;
        }

        log::info!("stance_score: {stance_score:?}");
        for topic in &stance_score.topics.unwrap_or_default() {
            topics_scores
                .entry(topic.to_string())
                .and_modify(|x| {
                    x.0.liberal += temp_score.liberal;
                    x.0.authoritarian += temp_score.authoritarian;
                    x.0.socialist += temp_score.socialist;
                    x.0.capitalist += temp_score.capitalist;
                    x.1 += 1;
                })
                .or_insert((temp_score, 1));
        }
    }

    Ok(topics_scores
        .into_iter()
        .map(|(topic, (score, count))| {
            let (pos_score, contra_score) = (
                score.socialist + score.liberal,
                score.authoritarian + score.capitalist,
            );
            StanceTopicScore {
                topic,
                score: -1.8 * (pos_score - contra_score) / count as f64,
                broken_down_score: score,
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use combx::connect_pg;

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
