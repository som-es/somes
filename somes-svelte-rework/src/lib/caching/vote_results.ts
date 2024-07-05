import { latest_vote_results } from "$lib/api";
import type { VoteResult } from "$lib/types";
import { get } from "svelte/store";
import { latestVoteResultsStore } from "./stores/stores";

export async function cachedLatestVoteResults(refetch: boolean = false): Promise<VoteResult[] | null> {
    let maybeCached = get(latestVoteResultsStore);
    if (maybeCached == null || refetch || maybeCached.length == 0) {
        const fetched = await latest_vote_results();
        latestVoteResultsStore.set(fetched);
        maybeCached = fetched;
    }
    return maybeCached;
}
