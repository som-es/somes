import type {
	VoteResultFilter,
	VoteResult,
	Delegate,
	GovPropFilter,
	DelegateFilter,
	DecreeFilter
} from '$lib/types';
import { persisted } from 'svelte-persisted-store';

export const currentDelegateStore = persisted<Delegate | null>('currentDelegate', null);
export const useCurrentDelegate = persisted<boolean>('currentVoteResult', false);
export const hasGoBackStore = persisted<boolean>('hasGoBack', false);
export const currentVoteResultStore = persisted<VoteResult | null>('currentVoteResult', null);
export const currentDelegatesAtDateStore = persisted<[string, Delegate[]] | null>(
	'currentDelegatesAtDate',
	null
);
export const currentVoteResultFilterStore = persisted<VoteResultFilter | null>(
	'currentVoteResultFilter',
	null
);
export const currentGovProposalFilterStore = persisted<GovPropFilter | null>(
	'currentGovPropFilter',
	null
);
export const currentDecreeFilterStore = persisted<DecreeFilter | null>(
	'currentDecreeFilter',
	null
);
export const currentDelegateFilterStore = persisted<DelegateFilter | null>(
	'currentDelegateFilter',
	null
);
