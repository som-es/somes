import type { LegisPeriod } from '$lib/types';
import { legisPeriodsStore } from './stores/stores.svelte';
import { all_gps, isHasError } from '$lib/api/api';
import { getParliament, type Parliament } from '$lib/api/parliament';

export async function cachedAllLegisPeriods(
	refetch: boolean = false,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<LegisPeriod[] | null> {
	let maybeCached = legisPeriodsStore.value;

	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await all_gps(fetcher, parliament);
		if (!isHasError(fetched)) {
			legisPeriodsStore.value = fetched;
			maybeCached = fetched;
		}
	}
	if (maybeCached !== null) {
		maybeCached = maybeCached.slice();
	}
	return maybeCached;
}
