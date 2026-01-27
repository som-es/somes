import { isHasError, latest_decrees, latest_ministrial_proposals, latest_vote_results } from "$lib/api/api";
import { fetchDelegates } from "$lib/api/fetch_delegates";
import { cachedAllSeats } from "$lib/caching/seats";
import { next_plenar_date } from "$lib/components/PlenarySessions/api";
import type { Delegate, HasError, VoteResult } from "$lib/types";
import type { PageServerLoad } from "./$types";

let internalCache: {
    data: any;
    timestamp: number;
} | null = null;

const CACHE_DURATION_MS = 1000 * 60 * 10;

async function fetchDelegatesFromVoteResult(latestVotes: VoteResult[] | HasError, fetcher: typeof fetch): Promise<Delegate[] | null> {
    if (isHasError(latestVotes)) {
        return []
    } 
    if (latestVotes.length == 0) return []
    const date = latestVotes[0].legislative_initiative.nr_plenary_activity_date;
    const gp = latestVotes[0].legislative_initiative.gp;
    const dels = await fetchDelegates(date, gp, fetcher)
    return dels.delegates
}

export const load: PageServerLoad = async ({ fetch, setHeaders }) => { 

    const now = Date.now();
    if (internalCache && (now - internalCache.timestamp < CACHE_DURATION_MS)) {
        // return internalCache.data;
    }
    if (process.env.NODE_ENV === 'production') {
        // setHeaders({
        //     'cache-control': 'max-age=1020'
        // });
    }

    // console.log("LOADING VOTE RESULTS");
    // const latestVotes = await latest_vote_results(fetch);
    // console.log("LOADED VOTE RESULTS");

    const [
        nextPlenarDate, 
        latestVotes,
        latestMinisterialProposals, 
        latestDecrees,
        allSeats
    ] = await Promise.all([
        next_plenar_date(fetch),
        latest_vote_results(fetch),
        latest_ministrial_proposals(30, fetch),
        latest_decrees(7, fetch),
        cachedAllSeats(false, fetch)
    ]);
    
    const delegates = fetchDelegatesFromVoteResult(latestVotes, fetch);

    // TODO error handling

    const data = {
        nextPlenarDate,
        latestVotes,
        latestMinisterialProposals,
        latestDecrees,
        delegates,
        allSeats
    };

    internalCache = {
        data,
        timestamp: now
    };

    return data;
}