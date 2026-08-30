import type { Vote, VoteResult } from './types';

/** all votes without ansence */
export function givenVotes(vote: Vote): number {
	return vote.infavor_count + vote.against_count + vote.abstention_count;
}

export function totalVotes(vote: Vote): number {
	return givenVotes(vote) + vote.absence_count;
}

export function isVoteInFavor(vote: Vote): boolean {
	return vote.infavor_count > vote.against_count;
}

export function isPartyInFavor(voteResult: VoteResult | null, party: string): boolean {
	const vote = voteResult?.votes.find((vote) => vote.party === party);
	if (!vote) {
		return false;
	}
	return isVoteInFavor(vote);
}

export function createPartyInfavorMap(
	voteResult: VoteResult | null,
	partyColors: Map<string, string>
): Map<string, boolean> {
	const partyToColorMap = partyColors;

	const partyInfavorMap = new Map<string, boolean>();
	partyToColorMap.forEach((_v, party) => {
		partyInfavorMap.set(party, isPartyInFavor(voteResult, party));
	});
	return partyInfavorMap;
}

/** sort decendings form size */
export function votesByPartySize(voteResult: VoteResult | null): Vote[] {
	return (voteResult?.votes ?? []).slice().sort((a, b) => totalVotes(b) - totalVotes(a));
}
