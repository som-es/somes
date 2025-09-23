import type { DbMinistrialProposalQuery } from '$lib/types';

export function createGovProposalPath(govProposal: DbMinistrialProposalQuery): string {
	return `/gov_proposal/${govProposal.gp}/ME/${govProposal.inr}`;
}
