import { delegates, delegates_at, delegates_with_seats_near_date, isHasError } from '$lib/api/api';
import { CircularBuffer } from '$lib/CircularBuffer';
import type { Delegate, DelegateSplit, HasError } from '$lib/types';
import { delegatesStore } from './stores/stores.svelte';

// create something that invalidates the cache every 30 minutes e.g.?
// local storage is not cleared everytime
export async function cachedDelegates(refetch: boolean = false): Promise<Delegate[] | null> {
	let dels = delegatesStore.value;
	if (dels == null || refetch || dels.length == 0) {
		const fetchedDels = await delegates();
		if (!isHasError(fetchedDels)) {
			delegatesStore.value = fetchedDels;
			dels = fetchedDels;
		}
	}
	return dels;
}

const delegatesNearDate: CircularBuffer<[string, string], Delegate[]> = new CircularBuffer(200);

export async function cachedDelegatesNearSeats(
	date: string,
	gp: string,
	refetch: boolean = false,
	fetcher: typeof fetch = fetch
): Promise<Delegate[] | null> {
	let dels = delegatesNearDate.findBy((e) => e[0] == date && e[1] == gp);
	if (dels == undefined || refetch || dels.length == 0) {
		console.log("NO DELEGATE CACHE");
		const fetchedDels = await delegates_with_seats_near_date(date as unknown as Date, gp, fetcher);
		if (isHasError(fetchedDels)) return null;
		delegatesNearDate.push([date, gp], fetchedDels);
		dels = fetchedDels;
	} else {
		console.log("DELEGATE CACHE");
	}
	return structuredClone($state.snapshot(dels.slice()));
}

export async function cachedDelegatedAtDate(date: string, gp: string, refetch: boolean = false, fetcher: typeof fetch = fetch): Promise<Delegate[] | HasError> {
	let dels = delegatesNearDate.findBy((e) => e[0] == date && e[1] == gp);
	if (dels == undefined || refetch || dels.length == 0) {
		const fetchedDels = await delegates_at(date, fetcher);
		if (isHasError(fetchedDels)) return fetchedDels;
		delegatesNearDate.push([date, gp], fetchedDels);
		dels = fetchedDels;
	}
	return structuredClone($state.snapshot(dels.slice()));
}

export function filterDelegates(dels: Delegate[]): DelegateSplit {
	let idx = 1;
	return dels.reduce<DelegateSplit>(
		(acc, delegate) => {
			const clonedDelegate = structuredClone($state.snapshot(delegate));
			if (clonedDelegate.council === 'nr') {
				acc.nr.push(clonedDelegate);
			} else if (clonedDelegate.council === 'gov') {
				acc.gov.push(clonedDelegate);
				// TODO FIXME some gps somehow have a lot of gov officials
				// INFO this is not required as gov officials stuff is the real stuff
				if (idx < 17) {
					clonedDelegate.seat_col = idx;
					clonedDelegate.seat_row = 7;
					idx += 1;
				} else {
					clonedDelegate.seat_col = null;
					clonedDelegate.seat_row = null;
				}
			}
			acc.all.push(clonedDelegate);

			return acc;
		},
		{ nr: [], gov: [], all: [] }
	);
}

export async function filteredDelegatesNearSeats(
	date: string,
	gp: string,
	refetch: boolean = false,
	fetcher: typeof fetch = fetch
): Promise<DelegateSplit | null> {
	const dels = await cachedDelegatesNearSeats(date, gp, refetch, fetcher);
	if (dels == null) {
		return null;
	}
	return filterDelegates(dels);
}

export async function filteredDelegates(refetch: boolean = false): Promise<DelegateSplit | null> {
	const dels = await cachedDelegates(refetch);
	if (dels == null) {
		return null;
	}
	return filterDelegates(dels);
}
