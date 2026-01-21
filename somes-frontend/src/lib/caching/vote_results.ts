import { isHasError, latest_vote_results } from '$lib/api/api';
import type { VoteResult } from '$lib/types';
import { latestVoteResultsStore } from './stores/stores.svelte';

export async function cachedLatestVoteResults(
	refetch: boolean = false
): Promise<VoteResult[] | null> {
	let maybeCached = latestVoteResultsStore.value;
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await latest_vote_results();
		if (isHasError(fetched)) {
			latestVoteResultsStore.value = null;
			maybeCached = null;
		} else {
			latestVoteResultsStore.value = fetched;
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
