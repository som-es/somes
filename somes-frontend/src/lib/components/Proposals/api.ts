import { getWithRoute } from '$lib/api/api';
import type { GovProposalDelegate, HasError } from '$lib/types';

export async function gov_proposal_by_path(
	gp: string,
	inr: string,
	fetcher: typeof fetch = fetch
): Promise<GovProposalDelegate | HasError> {
	return getWithRoute<GovProposalDelegate>(`v1/gov_proposals/${gp}/${inr}`, "at/", fetcher);
}
