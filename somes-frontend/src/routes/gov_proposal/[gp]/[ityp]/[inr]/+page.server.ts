import { vote_result_by_path } from '$lib/api/api';
import { gov_proposal_by_path } from '$lib/components/Proposals/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => { 

    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }

    const govProposal = await gov_proposal_by_path(params.gp, params.inr, fetch);

    return {
        govProposal 
    };
};


