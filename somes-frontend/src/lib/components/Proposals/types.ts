import type { DbMinistrialProposalQueryMeta } from '$lib/types';

export function createGovProposalPath(govProposal: DbMinistrialProposalQueryMeta): string {
	return `/gov_proposal/${govProposal.gp}/ME/${govProposal.inr}`;
}
