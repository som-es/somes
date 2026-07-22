import {
	cachedDelegatedAtDate,
	filterDelegates,
	filteredDelegatesNearSeats
} from '$lib/caching/delegates.svelte';
import { seatSettedCachedGovOfficials } from '$lib/caching/gov_officials';
import { delegates_at, errorToNull } from './api';
import { getParliament, type Parliament } from './parliament';

export async function fetchDelegates(
	dateStr: string,
	gp: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
) {
	let delegates = null;
	const hasSeatInfo = checkHasSeatInfo(dateStr, parliament);

	if (hasSeatInfo) {
		delegates = (await filteredDelegatesNearSeats(dateStr, gp, false, fetcher, parliament))?.nr || null;
	} else {
		const fetchedDelegates = errorToNull(
			await cachedDelegatedAtDate(dateStr, gp, false, fetcher, parliament)
		);
		if (fetchedDelegates) {
			const filteredDelegates = filterDelegates(fetchedDelegates);
			delegates = filteredDelegates.nr;
		}
	}
	if (delegates) {
		const govOfficials = (await seatSettedCachedGovOfficials(dateStr, fetcher, parliament)) ?? [];
		const allDelegates = delegates.concat(govOfficials);
		// Deduplicate by id
		// delegates = [...new Map(allDelegates.map((d) => [d.id, d])).values()];
		delegates = allDelegates;
	}
	return { hasSeatInfo, delegates };
}

function seatInfoAvailableAtDateForParliament(parliament: Parliament): string {
  switch (parliament) {
    case "at": return "2024-08-01"
    case "eu": return "2026-08-01"
  }
}

export function checkHasSeatInfo(dateStr: string, parliament: Parliament = getParliament()) {
	let hasSeatInfo = true;
	const date = new Date(dateStr);
	const seatInfoAvailableAtDate = seatInfoAvailableAtDateForParliament(parliament);
	if (date < new Date(seatInfoAvailableAtDate)) {
		hasSeatInfo = false;
	}
	return hasSeatInfo;
}
