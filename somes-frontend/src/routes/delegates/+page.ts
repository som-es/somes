import { delegate_by_id, errorToNull } from "$lib/api/api";
import { fetchDelegates } from "$lib/api/fetch_delegates";
import { cachedAllLegisPeriods } from "$lib/caching/legis_periods";
import { cachedAllSeats } from "$lib/caching/seats";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, url, setHeaders }) => { 
    const delegateId = url.searchParams.get("delegate");    
    const gp = url.searchParams.get("gp");    
    const date = url.searchParams.get("date") ?? new Date().toISOString().split('T')[0];

    const delegates = await fetchDelegates(date, gp ?? 'XXVIII', fetch);
	const cachedPeriods = (await cachedAllLegisPeriods())?.reverse();
	const cachedSeats = await cachedAllSeats();

    let delegate = null;

    if (delegateId) {
        delegate = errorToNull(await delegate_by_id(+delegateId, fetch));
    }


    setHeaders({
        'cache-control': 'max-age=120'
    });
    
    return { ...delegates, delegate, delegateId, cachedPeriods, gp, cachedSeats, date };
}