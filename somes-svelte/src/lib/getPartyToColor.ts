import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Readable } from "svelte/store";

export function getPartyToColor(): Map<string, string> {
    const partyColorStorage: Readable<string> = localStorageStore('parties', "");
    return new Map(JSON.parse(get(partyColorStorage)));
}

export function partyToColorFn(partyToColorMap: Map<string, string>, party: string): string {
    const color = partyToColorMap.get(party)
    if (color == null) {
        return "#B8B8B8";
    }
    return color;
}