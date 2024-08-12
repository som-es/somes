import type { VoteResult } from '$lib/types';
import { persisted } from 'svelte-persisted-store';

export const currentVoteResultStore = persisted<VoteResult | null>('currentVoteResult', null);
