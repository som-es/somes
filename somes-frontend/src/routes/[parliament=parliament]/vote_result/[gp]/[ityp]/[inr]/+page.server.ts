import { isHasError, vote_result_by_id } from '$lib/api/api';
import { vote_result_by_path } from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllSeats } from '$lib/caching/seats';
import type { Delegate, VoteResult } from '$lib/types';
import type { PageServerLoad } from './$types';
import { cachedPartyColors } from '$lib/caching/party_color';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => {
    const parliament = params.parliament as Parliament;

    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }
    const cachedSeats = await cachedAllSeats(false, fetch, parliament);
    const voteResult = await vote_result_by_path(params.gp, params.ityp, params.inr, fetch, parliament);

    let delegates: Delegate[] | null = null;
    let hasSeatInfo = true;
    let referencedByResults: VoteResult[] = [];
    let referencesResults: VoteResult[] = [];

    if (!isHasError(voteResult)) {
        const date = voteResult.legislative_initiative.vote_date !== null ? voteResult.legislative_initiative.vote_date : voteResult.legislative_initiative.nr_plenary_activity_date;
        ({ hasSeatInfo, delegates } = await fetchDelegates(date, params.gp, fetch, parliament));

        if (voteResult.referenced_by_others_ids.length > 0) {
            const results = await Promise.all(
                voteResult.referenced_by_others_ids.map(id => vote_result_by_id(id.toString(), fetch, parliament))
            );
            referencedByResults = results.filter(r => !isHasError(r)) as VoteResult[];
        }

        if (voteResult.references && voteResult.references.length > 0) {
            const results = await Promise.all(
                voteResult.references.map(ref => vote_result_by_path(ref.gp, ref.ityp, ref.inr.toString(), fetch, parliament))
            );
            referencesResults = results.filter(r => !isHasError(r)) as VoteResult[];
        }
    }

    const partyColors = await cachedPartyColors(true, parliament, fetch)

    return {
        voteResult, delegates, hasSeatInfo, cachedSeats, referencedByResults, referencesResults, parliament, partyColors
    };
};
