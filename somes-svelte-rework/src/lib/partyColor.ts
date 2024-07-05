import { parties } from "./api";


export function getPartyColors(): Map<string, string> {
    const partyColors = localStorage.getItem("partyColors")!;

	if (partyColors == "") {
		return new Map();
	}

	return new Map(JSON.parse(partyColors));
}

export function partyToColor(party: string | null): string {
	if (party == null) {
		return "#B8B8B8";
	}

	const color = getPartyColors().get(party);
	if (color == null) {
		return "#B8B8B8";
	}

	return color;
}
