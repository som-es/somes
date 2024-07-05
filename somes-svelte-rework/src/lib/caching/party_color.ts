import { parties } from "$lib/api";
import { get } from "svelte/store";
import { partyColorsStore } from "./stores/stores";

export async function cachedPartyColors(refetch: boolean = false): Promise<Map<string, string>> {
    let maybeCached = get(partyColorsStore);
    if (maybeCached == null || refetch) {
        let partyToColor = new Map<string, string>();
        (await parties()).forEach((party) => {
            partyToColor.set(party.name, party.color);
        });
        const colorsArray = Array.from(partyToColor.entries());
        partyColorsStore.set(colorsArray);
        maybeCached = colorsArray;
    }
    return new Map(maybeCached);
}
