import { latest_vote_results } from "$lib/api";
import type { VoteResult } from "$lib/types";
import { localStorageStore } from "@skeletonlabs/skeleton";
import { get, type Writable } from "svelte/store";

export async function cachedLatestVoteResults(refetch: boolean = false): Promise<VoteResult[]> {

    const store: Writable<VoteResult[] | null> = localStorageStore(
		"latest_vote_results",
		null,
	);
    let maybeCached = get(store);
    if (maybeCached == null || refetch) {
        const fetched = await latest_vote_results();
        store.set(fetched);
        maybeCached = fetched;
    }
    return maybeCached;
}
