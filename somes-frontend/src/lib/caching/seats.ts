import { seatsStore } from './stores/stores.svelte';
import { isHasError, seats } from '$lib/api/api';

export async function cachedAllSeats(
	refetch: boolean = false,
	fetcher: typeof fetch = fetch
): Promise<Map<string, number[]> | null> {
	let maybeCached = seatsStore.value;

	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await seats(fetcher);
		if (!isHasError(fetched)) {
			const colorsArray = Array.from(fetched.entries());
			maybeCached = colorsArray;
			seatsStore.value = colorsArray.slice();
		} else {
			maybeCached = [];
		}
	}
	return new Map(maybeCached);
}

export function getSeats(
	map: Map<string, number[]>,
	gp: string,
	forceBase: boolean = false
): number[] {
	const res = map.get(gp);
	if (res == null || forceBase) {
		return [18, 25, 29, 33, 37, 41];
	}
	return res;
}
