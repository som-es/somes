import { isHasError, latest_ministrial_proposals, latest_vote_results } from '$lib/api/api';
import type { GovProposalDelegate, VoteResult } from '$lib/types';
import { get } from 'svelte/store';
import { latestGovProposalsStore } from './stores/stores';

export async function cachedLatestGovProposals(
	refetch: boolean = false
): Promise<GovProposalDelegate[] | null> {
	let maybeCached = get(latestGovProposalsStore);
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await latest_ministrial_proposals(30);
		if (!isHasError(fetched)) {
			latestGovProposalsStore.set(fetched);
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
