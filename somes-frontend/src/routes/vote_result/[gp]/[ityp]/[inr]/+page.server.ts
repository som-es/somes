import { isHasError } from '$lib/api/api';
import { vote_result_by_path } from '$lib/api/api';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllSeats } from '$lib/caching/seats';
import type { Delegate} from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => { 

    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }
    const cachedSeats = await cachedAllSeats(false, fetch);
    const voteResult = await vote_result_by_path(params.gp, params.ityp, params.inr, fetch);

    let delegates: Delegate[] | null = null;
    let hasSeatInfo = true;
    if (!isHasError(voteResult)) {
        ({ hasSeatInfo, delegates } = await fetchDelegates(voteResult.legislative_initiative.nr_plenary_activity_date, params.gp, fetch));
    } 
    
    return {
        voteResult, delegates, hasSeatInfo, cachedSeats
    };
};


