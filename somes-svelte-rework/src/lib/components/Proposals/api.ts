import { getWithRoute } from '$lib/api/api';
import type { GovProposalDelegate, HasError } from '$lib/types';

export async function gov_proposal_by_path(
	gp: string,
	inr: string
): Promise<GovProposalDelegate | HasError> {
	return getWithRoute<GovProposalDelegate>(`gov_proposal/${gp}/${inr}`);
}
