import type { Delegate, NamedVote, VoteResult } from '$lib/types';
import { isVoteInFavor } from '$lib/partyInfavor';

export const NO_PARTY = 'NO_PARTY';

export interface SearchFilter {
	search: string;
	parties: string[];
	vote: string | null;
	countries: string[];
}

export interface SearchResult {
	delegate: Delegate;
	infavor: boolean | null;
	absent: boolean | null;
	abstention: boolean;
	// true = named vote, false = party vote
	isNamedVote: boolean;
}

export function partyOf(delegate: Delegate): string {
	return delegate.party?.trim() ? delegate.party : NO_PARTY;
}

export function searchDelegates(
	voteResult: VoteResult | null,
	delegates: Delegate[],
	filter: SearchFilter
): SearchResult[] {
	const search = filter.search.trim().toLowerCase();

	const namedVotes = new Map<number, NamedVote>();
	voteResult?.named_votes?.named_votes.forEach((v) => namedVotes.set(v.delegate_id, v));

	// filters delegate, appends vote info
	return delegates
		.filter((d) => search === '' || d.name.toLowerCase().includes(search))
		.filter((d) => filter.parties.length === 0 || filter.parties.includes(partyOf(d)))
		.filter((d) => filter.countries.length === 0 || filter.countries.includes(d.constituency))
		.map((d) => voteInfoOf(voteResult, namedVotes, d))
		.filter((result) => matchesVote(result, filter.vote));
}

function voteInfoOf(
	voteResult: VoteResult | null,
	namedVotes: Map<number, NamedVote>,
	delegate: Delegate
): SearchResult {
	const namedVote = namedVotes.get(delegate.id);
	if (namedVote) {
		return {
			delegate,
			infavor: namedVote.infavor,
			absent: namedVote.was_absent,
			abstention: namedVote.was_abstention,
			isNamedVote: true
		};
	}

	const partyVote = voteResult?.votes.find((v) => v.party === delegate.party);
	if (voteResult && partyVote) {
		const absent = voteResult.absences.includes(delegate.id);
		return {
			delegate,
			infavor: absent ? null : isVoteInFavor(partyVote),
			absent,
			abstention: false,
			isNamedVote: false
		};
	}

	return {
		delegate,
		infavor: null,
		absent: null,
		abstention: false,
		isNamedVote: false
	};
}

function matchesVote(result: SearchResult, vote: string | null): boolean {
	switch (vote) {
		case 'Infavor':
			return result.infavor === true;
		case 'Against':
			return result.infavor === false;
		case 'NoVote':
			return result.absent === true;
		case 'Abstention':
			return result.abstention === true;
		default:
			return true;
	}
}
