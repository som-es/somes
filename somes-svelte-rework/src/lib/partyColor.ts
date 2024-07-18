import { get } from 'svelte/store';
import { partyColorsStore } from './caching/stores/stores';

export function getPartyColors(): Map<string, string> {
	return new Map(get(partyColorsStore));
}

export function partyToColor(party: string | null): string {
	if (party == null) {
		return '#B8B8B8';
	}

	const color = getPartyColors().get(party);
	if (color == null) {
		return '#B8B8B8';
	}

	return color;
}
