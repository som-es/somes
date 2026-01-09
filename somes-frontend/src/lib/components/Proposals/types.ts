import { resolve } from '$app/paths';
import type { DbMinistrialProposalQueryMeta } from '$lib/types';

export function createGovProposalPath(govProposal: DbMinistrialProposalQueryMeta): string {
	return resolve(`/gov_proposal/[gp]/[ityp]/[inr]`, {gp: govProposal.gp, ityp: "ME", inr: govProposal.inr.toString()});
}
