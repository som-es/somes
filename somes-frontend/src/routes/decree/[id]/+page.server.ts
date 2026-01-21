import { delegate_by_id, errorToNull, isHasError, vote_result_by_path } from '$lib/api/api';
import { decree_by_ris_id } from '$lib/components/Delegates/Decrees/api';
import type { DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => { 

    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }

    const govDecree = await decree_by_ris_id(params.id, fetch);
    if (isHasError(govDecree)) return { decreeDelegate: null };



    const delegate = await delegate_by_id(govDecree.gov_official_id);
    if (isHasError(delegate)) return { decreeDelegate: null };

    let decreeDelegate: DecreeDelegate = { decree: govDecree, delegate: delegate };
    return {
        decreeDelegate 
    };
};


