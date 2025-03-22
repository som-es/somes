import { isHasError } from '$lib/api/api';
import { getFavoDelegates, getFavoLegisInits } from '$lib/api/authed';

export let favoDelegates: Set<number> | null = null;
export let favoLegisInits: Set<number> | null = null;

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

export async function cachedLegisInitFavos(
	refetch: boolean = false
): Promise<Set<number> | null> {
	let maybeCached = favoDelegates;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoLegisInits();
		if (!isHasError(fetched)) {
			maybeCached = new Set(fetched.map(x => x.vote_result_id));
		}
	}
	return maybeCached;
}