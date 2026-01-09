import { isHasError } from '$lib/api/api';
import { vote_result_by_path } from '$lib/api/api';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllSeats } from '$lib/caching/seats';
import type { Delegate} from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params, setHeaders }) => { 
    const cachedSeats = await cachedAllSeats();
    const voteResult = await vote_result_by_path(params.gp, params.ityp, params.inr, fetch);

    let delegates: Delegate[] | null = null;
    let hasSeatInfo = true;
    if (!isHasError(voteResult)) {
        ({ hasSeatInfo, delegates } = await fetchDelegates(voteResult.legislative_initiative.nr_plenary_activity_date, params.gp, fetch));
    }
    setHeaders({
        'cache-control': 'max-age=120'
    });
    
    return {
        voteResult, delegates, hasSeatInfo, cachedSeats
    };
};


