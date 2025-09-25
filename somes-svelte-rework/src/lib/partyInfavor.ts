import type { VoteResult } from './types';

export function isPartyInFavor(voteResult: VoteResult | null, party: string): boolean {
	const votes = voteResult?.votes.slice();
	if (!votes) {
		return false;
	}
	// this sort is there because of named votes -> it should only look at the one with the higher count (pro, contra)
	// otherwise, it could happen that (absent, or new) delegates are marked as e.g. contra delegates even though the majority of the party voted for the change
	votes.sort((a, b) => b.fraction - a.fraction);
	return votes.find((vote) => vote.party === party)?.infavor ?? false;
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
