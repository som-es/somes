import { delegates, delegates_with_seats_near_date, isHasError } from '$lib/api';
import { CircularBuffer } from '$lib/CircularBuffer';
import type { Delegate, DelegateSplit, HasError } from '$lib/types';
import { get } from 'svelte/store';
import { delegatesStore } from './stores/stores';

// create something that invalidates the cache every 30 minutes e.g.?
// local storage is not cleared everytime
export async function cachedDelegates(refetch: boolean = false): Promise<Delegate[] | null> {
	let dels = get(delegatesStore);
	if (dels == null || refetch || dels.length == 0) {
		const fetchedDels = await delegates();
		if (!isHasError(fetchedDels)) {
			delegatesStore.set(fetchedDels);
			dels = fetchedDels;
		}
	}
	return dels;
}

const delegatesNearDate: CircularBuffer<[string, string], Delegate[]> = new CircularBuffer(100);

export async function cachedDelegatesNearSeats(date: string, gp: string, refetch: boolean = false): Promise<Delegate[] | null> {
	let dels = delegatesNearDate.findBy((e) => e[0] == date && e[1] == gp);
	if (dels == undefined || refetch || dels.length == 0) {
		const fetchedDels = await delegates_with_seats_near_date(date as unknown as Date, gp);
		if (isHasError(fetchedDels)) return null;
		delegatesNearDate.push([date, gp], fetchedDels);
		dels = fetchedDels;
	}
	return structuredClone(dels.slice());
}

export function filterDelegates(dels: Delegate[]): DelegateSplit {
	const idx = 1;
	return dels.reduce<DelegateSplit>((acc, delegate) => {
		const clonedDelegate = structuredClone(delegate);
		if (clonedDelegate.council === 'nr') {
			acc.nr.push(clonedDelegate);
		} else if (clonedDelegate.council === 'gov') {
			acc.gov.push(clonedDelegate);
			clonedDelegate.seat_col = idx;
			clonedDelegate.seat_row = 7;
		}
		acc.all.push(clonedDelegate);
		
		return acc;
	}, { nr: [], gov: [], all: [] });
}

export async function filteredDelegatesNearSeats(date: string, gp: string, refetch: boolean = false): Promise<DelegateSplit | null> {
	const dels = await cachedDelegatesNearSeats(date, gp, refetch);
	if (dels == null) {
		return null;
	}
	return filterDelegates(dels)
}

export async function filteredDelegates(refetch: boolean = false): Promise<DelegateSplit | null> {
	const dels = await cachedDelegates(refetch);
	if (dels == null) {
		return null;
	}
	return filterDelegates(dels)
}
