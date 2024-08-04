import type { LegisPeriod } from "$lib/types";
import { get } from "svelte/store";
import { legisPeriodsStore } from "./stores/stores";
import { all_gps } from "$lib/api";

export async function cachedAllLegisPeriods(
	refetch: boolean = false
): Promise<LegisPeriod[] | null> {
	let maybeCached = get(legisPeriodsStore);
    
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await all_gps();
		legisPeriodsStore.set(fetched);
		maybeCached = fetched;
	}
    if (maybeCached !== null) {
        maybeCached = maybeCached.slice()
    }
	return maybeCached;
}
