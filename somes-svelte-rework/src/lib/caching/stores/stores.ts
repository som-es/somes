import type { Delegate, LegisPeriod, VoteResult } from '$lib/types';
import { persisted } from 'svelte-persisted-store';

export const partyColorsStore = persisted<[string, string][] | null>('partyColors', null);
export const delegatesStore = persisted<Delegate[] | null>('delegates', null);
export const latestVoteResultsStore = persisted<VoteResult[] | null>('latest_vote_results', null);
export const legisPeriodsStore = persisted<LegisPeriod[] | null>('legis_periods', null);
export const seatsStore = persisted<[string, number[]][] | null>('seats', null);
