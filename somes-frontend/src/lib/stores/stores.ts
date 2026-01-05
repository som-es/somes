import type { DecreeDelegate, DecreeFilter } from '$lib/components/Delegates/Decrees/types';
import { persisted } from '$lib/persisted.svelte';
import type {
	VoteResultFilter,
	VoteResult,
	Delegate,
	GovPropFilter,
	DelegateFilter,
	GovProposalDelegate
} from '$lib/types';

export const currentDelegateStore = persisted<Delegate | null>('currentDelegate', null);
export const useCurrentDelegate = persisted<boolean>('currentVoteResult', false);
export const hasGoBackStore = persisted<boolean>('hasGoBack', false);
export const currentVoteResultStore = persisted<VoteResult | null>('currentVoteResult', null);
export const currentDecreeStore = persisted<DecreeDelegate | null>('currentDecreeStore', null);
export const currentGovProposalDelegateStore = persisted<GovProposalDelegate | null>(
	'currentGovProposalDelegateStore',
	null
);
export const currentDelegatesAtDateStore = persisted<[string, Delegate[]] | null>(
	'currentDelegatesAtDate',
	null
);
export const currentVoteResultFilterStore = persisted<VoteResultFilter | null>(
	'currentVoteResultFilter',
	null
);

export const currentUnfinshedVoteResultFilterStore = persisted<VoteResultFilter | null>(
	'currentUnfinishedVoteResultFilter',
	null
);

export const currentVoteResultFilterStores = [
	currentVoteResultFilterStore,
	currentUnfinshedVoteResultFilterStore
];

export const currentGovProposalFilterStore = persisted<GovPropFilter | null>(
	'currentGovPropFilter',
	null
);
export const currentDecreeFilterStore = persisted<DecreeFilter | null>('currentDecreeFilter', null);
export const currentDelegateFilterStore = persisted<DelegateFilter | null>(
	'currentDelegateFilter',
	null
);
