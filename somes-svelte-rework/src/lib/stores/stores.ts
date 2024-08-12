import type { VoteResultFilter, VoteResult } from '$lib/types';
import { persisted } from 'svelte-persisted-store';

export const currentVoteResultStore = persisted<VoteResult | null>('currentVoteResult', null);
export const currentVoteResultFilterStore = persisted<VoteResultFilter | null>('currentVoteResultFilter', null);
