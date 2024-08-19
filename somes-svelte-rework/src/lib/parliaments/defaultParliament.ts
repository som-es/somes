import type { Bubble } from '$lib/parliament';
import type { Delegate } from '$lib/types';

export function groupPartyDelegates(dels: Delegate[]): Map<string, Delegate[]> {
	let partyToDelegates = new Map<string, Delegate[]>();

	for (let idx = 0; idx < dels.length; idx++) {
		const del = dels[idx];
		dels[idx].seat_row = null;
		dels[idx].seat_col = null;
		if (del.party == null || del.council != 'nr') {
			continue;
		}

		if (!partyToDelegates.has(del.party)) {
			partyToDelegates.set(del.party, []);
		}
		const currentDels = partyToDelegates.get(del.party);
		currentDels?.push(del);
	}
	return partyToDelegates;
}

export function setSeatsOfDels(
	partyToDelegatesArray: [string, Delegate[]][],
	all: number,
	defaultSeats: number[]
) {
	let startIdxs = [0, 0, 0, 0, 0, 0];
	partyToDelegatesArray.forEach(([party, dels], g) => {
		const fraction = dels.length;
		const share = fraction / all;
		let restSeats = 0;
		let startDelegateIdx = 0;
		defaultSeats.forEach((seats, r) => {
			let realSeats = Math.floor(seats * share);
			// console.log(realSeats);
			restSeats += seats * share - realSeats;

			if (realSeats + startIdxs[r] >= seats) {
				restSeats += realSeats + startIdxs[r] - seats;
				realSeats = seats - startIdxs[r];
			}
			const useDels = dels.slice(startDelegateIdx, startDelegateIdx + realSeats);

			useDels.forEach((del, c) => {
				del.seat_col = null;
				del.seat_row = null;
				del.seat_row = r + 1;
				del.seat_col = c + startIdxs[r] + 1;
			});

			startDelegateIdx += realSeats;
			startIdxs[r] += realSeats;
		});

		restSeats = Math.round(restSeats);

		let count = 0;
		let row = 0;

		// let row = defaultSeats.length -1;
        // for (let i = 0; i < startIdxs.length - 1; i++) {
        //     const first = startIdxs[i]
        //     const second = startIdxs[i + 1]
        //     if (first < second) {
        //         row = i;
        //         break;
        //     }
        // }
		while (true) {
			if (row >= defaultSeats.length) row = 0;
			// if (row <= 0) row = defaultSeats.length - 1;
			const seats = defaultSeats[row];

			if (count >= 1000) {
				// break
			}

			// break;

			count += 1;
			// console.log(restSeats);
			if (Math.round(restSeats) <= 0) {
				break;
			} else {
			}
			if (startIdxs[row] + 1 > seats) {
				row += 1;
				continue;
			}

			const del = dels[startDelegateIdx];
			if (del == null) break;
			del.seat_row = row + 1;
			del.seat_col = startIdxs[row] + 1;

			startDelegateIdx += 1;
			restSeats -= 1;
			startIdxs[row] += 1;
			row += 1;
		}
        return;
		// console.log(` ${party} ${restSeats}`);
	});
}
