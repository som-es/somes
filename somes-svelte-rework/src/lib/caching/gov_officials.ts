import { gov_officials_at, isHasError } from '$lib/api/api';
import { CircularBuffer } from '$lib/CircularBuffer';
import type { Delegate } from '$lib/types';

const govOfficialsAtDate: CircularBuffer<string, Delegate[]> = new CircularBuffer(100);

export async function cachedGovOfficials(
	date: string,
	refetch: boolean = false
): Promise<Delegate[] | null> {
	let dels = govOfficialsAtDate.findBy((e) => e == date);
	if (dels == undefined || refetch || dels.length == 0) {
		const fetchedDels = await gov_officials_at(date as unknown as Date);
		if (isHasError(fetchedDels)) return null;
		govOfficialsAtDate.push(date, fetchedDels);
		dels = fetchedDels;
	}
	return structuredClone(dels.slice());
}

export async function seatSettedCachedGovOfficials(date: string): Promise<Delegate[] | null> {
	const dels: Delegate[] | null = await cachedGovOfficials(date);
	if (dels == null) {
		return null;
	}
	return dels.map((delegate, idx) => {
		// TODO FIXME some gps somehow have a lot of gov officials
		// with this thing, not all gov officials are shown
		if (idx < 18) {
			delegate.seat_col = idx + 1;
			delegate.seat_row = 7;
		}
		return delegate;
	});
}
