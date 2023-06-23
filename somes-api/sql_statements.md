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