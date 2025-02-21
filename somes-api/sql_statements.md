```sql
select 
    delegates.name, 
    delegates.image_url,
    delegates.party, 
    cast(SUM(
    plenar_speeches.duration_in_seconds
    ) / (60.0 * 60.0) as float) AS hours_spoken 
from 
    plenar_speeches 
    inner join debates on plenar_speeches.debate_id = debates.id 
    inner join plenar_infos on debates.plenar_id = plenar_infos.id 
    inner join delegates on delegates.id = plenar_speeches.delegate_id 
where 
    plenar_infos.legislative_period = 'XXVII' 
    and delegates.council = 'nr'
group by 
    plenar_speeches.delegate_id, 
    delegates.image_url, 
    delegates.name, 
    delegates.party, 
    delegates.council 
order by 
    hours_spoken DESC;
```

```sql
select delegates.name,delegates.image_url,delegates.party,COUNT(*) as amount from call_to_order inner join delegates on call_to_order.receiver_id=delegates.id group by delegates.name,delegates.image_url,delegates.party,call_to_order.receiver_id order by amount DESC;
```

```sql
select 
        delegates.name, 
        delegates.image_url,
        delegates.party, 
        cast(
        plenar_speeches.duration_in_seconds
         / (60.0 * 60.0) as real) AS hours_spoken 
    from 
        plenar_speeches 
        inner join debates on plenar_speeches.debate_id = debates.id 
        inner join plenar_infos on debates.plenar_id = plenar_infos.id 
        inner join delegates on delegates.id = plenar_speeches.delegate_id 
    where 
        delegates.name like '%Kogler%'
        and delegates.council = 'nr'
    order by 
        hours_spoken DESC;
```


```sql

SELECT 
    d.id,
    d.name,
    SUM(
        CASE 
            WHEN p.ityp = 'J' THEN 1 
            WHEN p.ityp = 'AA' THEN 1.2 * proposal_count
            WHEN p.ityp = 'A' THEN 1.2 * proposal_count
            WHEN p.ityp = 'UEA' THEN 1.15 * proposal_count
            WHEN p.ityp = 'I' THEN 1.3 * proposal_count
            ELSE 0
        END
    ) AS activity_score
FROM 
    proposals p
JOIN 
    proposal_delegates pd ON p.id = pd.proposal_id
JOIN 
    delegates d ON pd.delegate_id = d.id
LEFT JOIN (
    SELECT 
        p.id AS proposal_id,
        COUNT(p.id) AS proposal_count
    FROM 
        proposals p
    JOIN 
        proposal_delegates pd ON p.id = pd.proposal_id
    WHERE 
        pd.is_receiver = false
    GROUP BY 
        p.id
) AS proposal_counts ON p.id = proposal_counts.proposal_id
WHERE 
    pd.is_receiver = false
GROUP BY 
    d.id
ORDER BY 
    activity_score DESC;

```