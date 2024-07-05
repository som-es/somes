import { delegates } from "$lib/api";
import type { Delegate } from "$lib/types";
import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Writable } from "svelte/store";

// create something that invalidates the cache every 30 minutes e.g.?
// local storage is not cleared everytime
export async function cachedDelegates(refetch: boolean = false): Promise<Delegate[]> {

    const store: Writable<Delegate[] | null> = localStorageStore(
		"delegates",
		null,
	);
    let dels = get(store);
    if (dels == null || refetch) {
        const fetchedDels = await delegates();
        store.set(fetchedDels);
        dels = fetchedDels;
    }
    return dels;
}
