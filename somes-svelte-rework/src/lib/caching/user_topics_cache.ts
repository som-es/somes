import { isHasError, latest_vote_results } from '$lib/api/api';
import type { UniqueTopic, VoteResult } from '$lib/types';
import { get } from 'svelte/store';
import { latestVoteResultsStore, userTopicsStore } from './stores/stores';
import { getUserTopics } from '$lib/api/authed';

export async function cachedUserTopics(
	refetch: boolean = false
): Promise<UniqueTopic[] | null> {
	let maybeCached = get(userTopicsStore);
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await getUserTopics();
		if (!isHasError(fetched)) {
			userTopicsStore.set(fetched);
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
