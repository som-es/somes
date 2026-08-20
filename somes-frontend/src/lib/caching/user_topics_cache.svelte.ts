import { isHasError } from '$lib/api/api';
import type { UniqueTopic } from '$lib/types';
import { userTopicsStore } from './stores/stores.svelte';
import { getUserTopics } from '$lib/api/authed';
import { getParliament, type Parliament } from '$lib/api/parliament';

export async function cachedUserTopics(refetch: boolean = false,
    parliament: Parliament = getParliament(),
): Promise<UniqueTopic[] | null> {
	let maybeCached = userTopicsStore.value;
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await getUserTopics();
		if (!isHasError(fetched)) {
			userTopicsStore.value = fetched;
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
