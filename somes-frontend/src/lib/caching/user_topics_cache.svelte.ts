import { isHasError } from '$lib/api/api';
import type { UniqueTopic } from '$lib/types';
import { userTopicsStore } from './stores/stores.svelte';
import { getUserTopics } from '$lib/api/authed';

export async function cachedUserTopics(refetch: boolean = false): Promise<UniqueTopic[] | null> {
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
