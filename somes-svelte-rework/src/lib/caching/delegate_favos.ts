import { isHasError } from '$lib/api/api';
import type { DelegateFavo } from '$lib/types';
import { getFavoDelegates } from '$lib/api/authed';

export let favoDelegates: Set<number> | null = null;

export async function cachedDelegateFavos(
	refetch: boolean = false
): Promise<Set<number> | null> {
	let maybeCached = favoDelegates;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoDelegates();
		if (!isHasError(fetched)) {
			maybeCached = new Set(fetched.map(x => x.delegate_id));
		}
	}
	return maybeCached;
}
