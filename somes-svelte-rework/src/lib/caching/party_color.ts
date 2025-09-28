import { isHasError, parties } from '$lib/api/api';
import { get } from 'svelte/store';
import { partyColorsStore } from './stores/stores';
import { setPartyColors } from '$lib/partyColor';

export async function cachedPartyColors(refetch: boolean = false): Promise<Map<string, string>> {
	let maybeCached = get(partyColorsStore);
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const partyToColor = new Map<string, string>();
		const partiesResult = await parties();
		if (!isHasError(partiesResult)) {
			partiesResult.forEach((party) => {
				partyToColor.set(party.name, party.color);
			});
		}

		const colorsArray = Array.from(partyToColor.entries());
		partyColorsStore.set(colorsArray);
		setPartyColors(new Map(colorsArray));
		maybeCached = colorsArray;
	}
	return new Map(maybeCached);
}
