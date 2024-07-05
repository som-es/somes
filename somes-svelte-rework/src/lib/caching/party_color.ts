import { parties } from "$lib/api";
import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Writable } from "svelte/store";

export async function cachedPartyColors(refetch: boolean = false): Promise<Map<string, string>> {
    const store: Writable<Map<string, string> | null> = localStorageStore(
		"partyColors",
		null,
	);
    let maybeCached = get(store);
    if (maybeCached == null || refetch) {
        let partyToColor = new Map<string, string>();
        (await parties()).forEach((party) => {
            partyToColor.set(party.name, party.color);
        });
        store.set(partyToColor);
        maybeCached = partyToColor;
    }
    return maybeCached;
}
