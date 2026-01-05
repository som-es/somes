import { gov_officials_at, isHasError } from '$lib/api/api';
import { CircularBuffer } from '$lib/CircularBuffer';
import { AMOUNT_PER_SIDE } from '$lib/parliament';
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

	const leftDels = dels.slice(0, dels.length / 2);
	const rightDels = dels.slice(dels.length / 2);

	for (let idx = 0; idx < AMOUNT_PER_SIDE; idx++) {
		if (idx < leftDels.length) {
			leftDels[idx].seat_col = AMOUNT_PER_SIDE - idx -1
			leftDels[idx].seat_row = 7 
		}

		if (idx < rightDels.length) {
			rightDels[idx].seat_col = AMOUNT_PER_SIDE + idx 
			rightDels[idx].seat_row = 7 
		}
	}

	return dels;

}
