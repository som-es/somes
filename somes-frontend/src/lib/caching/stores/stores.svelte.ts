import { persisted, persistedScoped } from '$lib/persisted.svelte';
import type {
	Delegate,
	DelegateFavo,
	GovProposalDelegate,
	LegisPeriod,
	UniqueTopic,
	VoteResult
} from '$lib/types';

export const partyColorsStore = persistedScoped<[string, string][] | null>('partyColors', null);
export const delegatesStore = persistedScoped<Delegate[] | null>('delegates', null);
export const govOfficialsStore = persistedScoped<Delegate[] | null>('gov_officials', null);
export const latestVoteResultsStore = persistedScoped<VoteResult[] | null>(
	'latest_vote_results',
	null
);
export const latestGovProposalsStore = persistedScoped<GovProposalDelegate[] | null>(
	'latest_ministrial_proposals',
	null
);
export const legisPeriodsStore = persistedScoped<LegisPeriod[] | null>('legis_periods', null);
export const seatsStore = persistedScoped<[string, number[]][] | null>('seats', null);
export const jwtStore = persisted<string | null>('access_token', null);
export const jwtQuizStore = persisted<string | null>('access_token_quiz', null);
export const userTopicsStore = persisted<UniqueTopic[] | null>('user_topics', null);
export const userDelegateFavosStore = persistedScoped<DelegateFavo[] | null>(
	'delegate_favos',
	null
);

export const loginDrawerOpenStore = persisted<boolean>('login_drawer', false);
