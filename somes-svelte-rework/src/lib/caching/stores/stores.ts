import type { Delegate, VoteResult } from '$lib/types';
import { persisted } from 'svelte-persisted-store'

export const partyColorsStore = persisted<[string, string][] | null>("partyColors", null);
export const delegatesStore = persisted<Delegate[] | null>("delegates", null);
export const latestVoteResultsStore = persisted<VoteResult[] | null>("latest_vote_results", null);
