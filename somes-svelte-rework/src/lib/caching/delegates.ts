import { delegates } from "$lib/api";
import type { Delegate } from "$lib/types";
import { get } from "svelte/store";
import { delegatesStore } from "./stores/stores";

// create something that invalidates the cache every 30 minutes e.g.?
// local storage is not cleared everytime
export async function cachedDelegates(refetch: boolean = false): Promise<Delegate[]> {
    let dels = get(delegatesStore);
    if (dels == null || refetch) {
        const fetchedDels = await delegates();
        delegatesStore.set(fetchedDels);
        dels = fetchedDels;
    }
    return dels;
}
