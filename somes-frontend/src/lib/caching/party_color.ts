import { isHasError, parties } from '$lib/api/api';
import { partyColorsStore } from './stores/stores.svelte';
import { setPartyColors } from '$lib/partyColor';

export async function cachedPartyColors(refetch: boolean = false): Promise<Map<string, string>> {
	let maybeCached = partyColorsStore.value;
	if (maybeCached == null || refetch || maybeCached.length == 0) {
		const partyToColor = new Map<string, string>();
		const partiesResult = await parties();
		if (!isHasError(partiesResult)) {
			partiesResult.forEach((party) => {
				partyToColor.set(party.name, party.color);
			});
		}

		const colorsArray = Array.from(partyToColor.entries());
		partyColorsStore.value = colorsArray;
		setPartyColors(new Map(colorsArray));
		maybeCached = colorsArray;
	}
	return new Map(maybeCached);
}
