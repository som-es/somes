use somes_common_lib::InterestShare;
use sqlx::PgPool;

pub async fn extract_interests_of_delegate(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<Vec<InterestShare>> {
    let absolute_interests = sqlx::query!("select 
        topic, COUNT(*) as talk_count from speeches 
            inner join topics_legis_init on topics_legis_init.legislative_initiatives_id=speeches.legislative_initiatives_id 
            inner join delegates on speeches.delegate_id = delegates.id 
        where infavor is not null and delegates.id = $1 
            group by topic
        order by topic;", delegate_id).fetch_all(pg).await?;

    let total_talk_counts = sqlx::query!("
        select topic, COUNT(*) as talk_count from speeches 
        inner join topics_legis_init on topics_legis_init.legislative_initiatives_id=speeches.legislative_initiatives_id inner join delegates on speeches.delegate_id = delegates.id 
        where infavor is not null and delegates.council = 'nr' and is_active group by topic order by topic;").fetch_all(pg).await?;

    let mut interest_shares = Vec::with_capacity(total_talk_counts.len());

    let talk_count_sum = absolute_interests
        .iter()
        .map(|val| val.talk_count.unwrap())
        .sum::<i64>();

    for (absolute_interest, absolute_talk_count) in absolute_interests
        .into_iter()
        .zip(total_talk_counts.into_iter())
    {
        let share_on_total = absolute_interest.talk_count.unwrap() as f32
            / absolute_talk_count.talk_count.unwrap() as f32;
        let share_on_self = absolute_interest.talk_count.unwrap() as f32 / talk_count_sum as f32;

        interest_shares.push(InterestShare {
            topic: absolute_interest.topic,
            total_share: share_on_total,
            self_share: share_on_self,
        });
    }

    Ok(interest_shares)
}

#[cfg(test)]
mod tests {
    use sqlx::postgres::PgPoolOptions;

    use crate::DATASERVICE_URL;

    #[tokio::test]
    async fn test_extract_interests_of_delegate() {
        let pg_pool = PgPoolOptions::new()
            .max_connections(200)
            .connect(DATASERVICE_URL)
            .await
            .unwrap();

        let mut interests = crate::routes::extract_interests_of_delegate(35520, &pg_pool)
            .await
            .unwrap();
        interests.sort_by(|a, b| b.self_share.total_cmp(&a.self_share));
        println!("interests: {interests:?}");
    }
}
