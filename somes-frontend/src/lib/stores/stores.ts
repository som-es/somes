import type { DecreeDelegate, DecreeFilter } from '$lib/components/Delegates/Decrees/types';
import { persisted, persistedScoped } from '$lib/persisted.svelte';
import type {
	VoteResultFilter,
	VoteResult,
	Delegate,
	GovPropFilter,
	DelegateFilter,
	GovProposalDelegate
} from '$lib/types';

export const currentDelegateStore = persistedScoped<Delegate | null>('currentDelegate', null);
export const useCurrentDelegate = persistedScoped<boolean>('currentVoteResult', false);
export const hasGoBackStore = persisted<boolean>('hasGoBack', false);
export const currentVoteResultStore = persistedScoped<VoteResult | null>('currentVoteResult', null);
export const currentDecreeStore = persistedScoped<DecreeDelegate | null>(
	'currentDecreeStore',
	null
);
export const currentGovProposalDelegateStore = persistedScoped<GovProposalDelegate | null>(
	'currentGovProposalDelegateStore',
	null
);
export const currentDelegatesAtDateStore = persistedScoped<[string, Delegate[]] | null>(
	'currentDelegatesAtDate',
	null
);
export const currentVoteResultFilterStore = persistedScoped<VoteResultFilter | null>(
	'currentVoteResultFilter',
	null
);

export const currentUnfinshedVoteResultFilterStore = persistedScoped<VoteResultFilter | null>(
	'currentUnfinishedVoteResultFilter',
	null
);

export const currentVoteResultFilterStores = [
	currentVoteResultFilterStore,
	currentUnfinshedVoteResultFilterStore
];

export const currentGovProposalFilterStore = persistedScoped<GovPropFilter | null>(
	'currentGovPropFilter',
	null
);
export const currentDecreeFilterStore = persistedScoped<DecreeFilter | null>(
	'currentDecreeFilter',
	null
);
export const currentDelegateFilterStore = persistedScoped<DelegateFilter | null>(
	'currentDelegateFilter',
	null
);

export const aiViewEnabledStore = persisted<boolean>('aiViewEnabled', true);
