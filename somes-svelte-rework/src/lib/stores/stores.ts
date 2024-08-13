import type { VoteResultFilter, VoteResult, Delegate } from '$lib/types';
import { persisted } from 'svelte-persisted-store';

export const currentDelegateStore = persisted<Delegate | null>('currentDelegate', null);
export const hasGoBackStore = persisted<boolean>('hasGoBack', false);
export const currentVoteResultStore = persisted<VoteResult | null>('currentVoteResult', null);
export const currentVoteResultFilterStore = persisted<VoteResultFilter | null>(
	'currentVoteResultFilter',
	null
);
