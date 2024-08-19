import { delegates } from '$lib/api';
import type { Delegate, HasError } from '$lib/types';
import { get } from 'svelte/store';
import { delegatesStore } from './stores/stores';

// create something that invalidates the cache every 30 minutes e.g.?
// local storage is not cleared everytime
export async function cachedDelegates(refetch: boolean = false): Promise<Delegate[] | null> {
	let dels = get(delegatesStore);
	if (dels == null || refetch || dels.length == 0) {
		const fetchedDels = await delegates();
		delegatesStore.set(fetchedDels);
		dels = fetchedDels;
	}
	return dels;
}

export async function filteredDelegates(refetch: boolean = false): Promise<Delegate[] | null> {
	const dels = await cachedDelegates(refetch);
	if (dels == null) {
		return null;
	}
	return structuredClone(dels.filter((delegate) => delegate.council === 'nr'));
}
