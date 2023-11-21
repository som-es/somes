use somes_common_lib::InterestShare;
use sqlx::PgPool;

pub async fn extract_interests_of_delegate(delegate_id: i32, pg: &PgPool) -> sqlx::Result<()> {
    let absolute_interests = sqlx::query!("select 
        topic, COUNT(*) as talk_count from speeches 
            inner join topics on topics.legislative_initiatives_id=speeches.legislative_initiatives_id 
            inner join delegates on speeches.delegate_id = delegates.id 
        where infavor is not null and delegates.id = $1 
            group by topic
        order by topic;", delegate_id).fetch_all(pg).await?;

    let total_talk_counts = sqlx::query!("
        select topic, COUNT(*) as talk_count from speeches 
        inner join topics on topics.legislative_initiatives_id=speeches.legislative_initiatives_id inner join delegates on speeches.delegate_id = delegates.id 
        where infavor is not null and delegates.council = 'nr' and is_active group by topic order by topic;").fetch_all(pg).await?;

    let mut interest_shares = Vec::with_capacity(total_talk_counts.len());

    for (absolute_interest, absolute_talk_count) in absolute_interests.into_iter().zip(total_talk_counts.into_iter()) {
        let share_on_total = absolute_interest.talk_count.unwrap() as f32 / absolute_talk_count.talk_count.unwrap() as f32;
        interest_shares.push(InterestShare {
            topic: absolute_interest.topic,
            interest_share: share_on_total
        });
    }


    Ok(())
}
