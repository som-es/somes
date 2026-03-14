import { isHasError } from '$lib/api/api';
import { getFavoDelegates, getFavoLegisInits } from '$lib/api/authed';
import { SvelteSet } from 'svelte/reactivity';

export let favoDelegates: SvelteSet<number> | null = null;
export let favoLegisInits: SvelteSet<number> | null = null;

export async function cachedDelegateFavos(refetch: boolean = false): Promise<SvelteSet<number> | null> {
	let maybeCached = favoDelegates;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoDelegates();
		if (!isHasError(fetched)) {
			maybeCached = new SvelteSet(fetched.map((x) => x.delegate_id));
		}
	}
	return maybeCached;
}

export async function cachedLegisInitFavos(refetch: boolean = false): Promise<SvelteSet<number> | null> {
	let maybeCached = favoDelegates;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoLegisInits();
		if (!isHasError(fetched)) {
			maybeCached = new SvelteSet(fetched.map((x) => x.vote_result_id));
		}
	}
	return maybeCached;
}
