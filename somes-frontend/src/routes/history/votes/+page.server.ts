import { parties_per_gp, vote_results_by_query_search, vote_results_by_search } from "$lib/api/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, params, setHeaders, url }) => {
    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }
    
    const queryParams = url.searchParams.toString();
    const filter = `${queryParams}&is_finished=true`
    const voteResults = await vote_results_by_query_search(filter, fetch);
    const partiesPerGp = await parties_per_gp(fetch);
    return { voteResults, partiesPerGp, selectedGp: null }
}
