import { isHasError, latest_ministrial_proposals, latest_vote_results } from '$lib/api/api';
import type { GovProposalDelegate, VoteResult } from '$lib/types';
import { latestGovProposalsStore } from './stores/stores.svelte';

export async function cachedLatestGovProposals(
	refetch: boolean = false
): Promise<GovProposalDelegate[] | null> {
	let maybeCached = latestGovProposalsStore.value;
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await latest_ministrial_proposals(30);
		if (!isHasError(fetched)) {
			latestGovProposalsStore.value = fetched;
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
