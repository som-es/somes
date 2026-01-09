import { latest_ministrial_proposals, latest_vote_results } from "$lib/api/api";
import { next_plenar_date } from "$lib/components/PlenarySessions/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, setHeaders }) => { 

    const nextPlenarDate = await next_plenar_date(fetch)
    const latestVotes = await latest_vote_results(fetch);
    const latestMinisterialProposals = await latest_ministrial_proposals(14, fetch);
    
    setHeaders({
        'cache-control': 'max-age=120'
    });
}