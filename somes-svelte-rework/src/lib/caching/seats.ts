import { get } from 'svelte/store';
import { seatsStore } from './stores/stores';
import { isHasError, seats } from '$lib/api';

export async function cachedAllSeats(
	refetch: boolean = false
): Promise<Map<string, number[]> | null> {
	let maybeCached = get(seatsStore);

	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const fetched = await seats();
		if (!isHasError(fetched)) {
			const colorsArray = Array.from(fetched.entries());
			maybeCached = colorsArray;
			seatsStore.set(colorsArray.slice())
		} else {
			maybeCached = []
		}
	}
	return new Map(maybeCached);
}

export function getSeats(map: Map<string, number[]>, gp: string, forceBase: boolean = false): number[] {
	const res = map.get(gp)
	if (res == null || forceBase) {
		return [18, 25, 29, 33, 37, 41];
	}
	return res
}
