import { isHasError } from '$lib/api/api';
import type { DelegateFavo, UniqueTopic } from '$lib/types';
import { get } from 'svelte/store';
import { userDelegateFavosStore as delegateFavoStore } from './stores/stores';
import { getFavoDelegates } from '$lib/api/authed';

export async function cachedDelegateFavos(
	refetch: boolean = false
): Promise<DelegateFavo[] | null> {
	let maybeCached = get(delegateFavoStore);
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await getFavoDelegates();
		if (!isHasError(fetched)) {
			delegateFavoStore.set(fetched);
			maybeCached = fetched;
		}
	}
	return maybeCached;
}
