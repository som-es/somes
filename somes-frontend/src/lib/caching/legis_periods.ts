import type { LegisPeriod } from '$lib/types';
import { legisPeriodsStore } from './stores/stores.svelte';
import { all_gps, isHasError } from '$lib/api/api';

export async function cachedAllLegisPeriods(
	refetch: boolean = false
): Promise<LegisPeriod[] | null> {
	let maybeCached = legisPeriodsStore.value;

	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await all_gps();
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
