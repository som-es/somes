import { gov_proposals_by_search, parties_per_gp, vote_results_by_query_search } from "$lib/api/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, setHeaders, url }) => {
    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }
    
    const searchParams = url.searchParams;
    if (searchParams.get("page") == null) {
        searchParams.set("page", "1")
    }
    const queryParams = searchParams.toString();
    const filter = `${queryParams}`
    const govProposals = await gov_proposals_by_search(filter, fetch);
    return { govProposals, selectedGp: null }
}
