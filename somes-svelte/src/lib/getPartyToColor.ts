import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Readable } from "svelte/store";

export function getPartyToColor(): Map<string, string> {
    const partyColorStorage: Readable<string> = localStorageStore('parties', "");
    return new Map(JSON.parse(get(partyColorStorage)));
}