import { parties } from "./api";

export async function updateColorStorage() {
    let partyToColor = new Map<string, string>();
    (await parties()).forEach((party) => {
        partyToColor.set(party.name, party.color);
    });
    localStorage.setItem("partyColors", JSON.stringify(Array.from(partyToColor.entries())));
}

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
