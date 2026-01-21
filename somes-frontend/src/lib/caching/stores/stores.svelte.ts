import { persisted } from '$lib/persisted.svelte';
import type {
	Delegate,
	DelegateFavo,
	GovProposalDelegate,
	LegisPeriod,
	UniqueTopic,
	VoteResult
} from '$lib/types';

export const partyColorsStore = persisted<[string, string][] | null>('partyColors', null);
export const delegatesStore = persisted<Delegate[] | null>('delegates', null);
export const govOfficialsStore = persisted<Delegate[] | null>('gov_officials', null);
export const latestVoteResultsStore = persisted<VoteResult[] | null>('latest_vote_results', null);
export const latestGovProposalsStore = persisted<GovProposalDelegate[] | null>(
	'latest_ministrial_proposals',
	null
);
export const legisPeriodsStore = persisted<LegisPeriod[] | null>('legis_periods', null);
export const seatsStore = persisted<[string, number[]][] | null>('seats', null);
export const jwtStore = persisted<string | null>('access_token', null);
export const jwtQuizStore = persisted<string | null>('access_token_quiz', null);
export const userTopicsStore = persisted<UniqueTopic[] | null>('user_topics', null);
export const userDelegateFavosStore = persisted<DelegateFavo[] | null>('delegate_favos', null);
