import { plink } from '$lib/api/parliament';
import type { DbMinistrialProposalQueryMeta } from '$lib/types';

export function createGovProposalPath(govProposal: DbMinistrialProposalQueryMeta): string {
	return plink(`/gov_proposal/${govProposal.gp}/ME/${govProposal.inr}`);
}
