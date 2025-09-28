import { get } from 'svelte/store';
import { partyColorsStore } from './caching/stores/stores';

const staticPartyColors: [string, string][] = [
	['BZÖ', '#FF8D04'],
	['F', '#0052FB'],
	['F-BZÖ', '#0052FB'],
	['FPÖ', '#0052FB'],
	['GRÜNE', '#69B12E'],
	['JETZT', '#C8C8C8'],
	['L', '#B0D8F3'],
	['NEOS', '#E3257B'],
	['NEOS-LIF', '#E3257B'],
	['ÖVP', '#62C3D0'],
	['PILZ', '#C8C8C8'],
	['SPÖ', '#FF0000'],
	['STRONACH', '#F8E924']
];

export function getPartyColors(): Map<string, string> {
	let storedPartyColors = get(partyColorsStore);
	if (!storedPartyColors || (storedPartyColors !== null && storedPartyColors.length == 0)) {
		storedPartyColors = staticPartyColors;
	}
	return new Map(storedPartyColors);
}

export let partyColors: Map<string, string> = getPartyColors();

export function setPartyColors(val: Map<string, string>) {
	partyColors = val;
}

export function partyToColor(party: string | null): string {
	if (party == null) {
		return '#B8B8B8';
	}

	// const color = getPartyColors().get(party);
	const color = partyColors.get(party);
	if (color == null) {
		return '#B8B8B8';
	}

	return color;
}
