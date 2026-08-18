import { isHasError } from '$lib/api/api';
import { getFavoDelegates, getFavoLegisInits } from '$lib/api/authed';
import { getParliament, type Parliament } from '$lib/api/parliament';
import type { DelegateFavo } from '$lib/types';
import { SvelteMap, SvelteSet } from 'svelte/reactivity';

export let favoDelegates: SvelteMap<number, DelegateFavo> | null = null;
export let favoLegisInits: SvelteSet<number> | null = null;

export async function cachedDelegateFavos(
	refetch: boolean = false,
	parliament: Parliament = getParliament(),
): Promise<SvelteMap<number, DelegateFavo> | null> {
	let maybeCached = favoDelegates;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoDelegates(parliament);
		if (!isHasError(fetched)) {
			maybeCached = new SvelteMap(fetched.map((favo) => [favo.delegate_id, favo]));
		}
	}
	return maybeCached;
}

export async function cachedLegisInitFavos(
	refetch: boolean = false,
	parliament: Parliament = getParliament(),
): Promise<SvelteSet<number> | null> {
	let maybeCached = favoLegisInits;
	if (maybeCached == null || refetch || maybeCached.size == 0) {
		const fetched = await getFavoLegisInits(parliament);
		if (!isHasError(fetched)) {
			maybeCached = new SvelteSet(fetched.map((x) => x.vote_result_id));
		}
	}
	return maybeCached;
}
