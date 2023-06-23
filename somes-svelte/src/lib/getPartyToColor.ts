import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Readable } from "svelte/store";

export function getPartyToColor(): Map<string, string> {
    const partyColors: string = get(localStorageStore('parties', ""));
    if (partyColors == "") {
        return new Map()
    }
    return new Map(JSON.parse(partyColors));
}

export function partyToColorFn(partyToColorMap: Map<string, string>, party: string | null): string {
    if (party == null) {
        return "#B8B8B8";
    }
    
    const color = partyToColorMap.get(party)
    if (color == null) {
        return "#B8B8B8";
    }
    return color;
}
