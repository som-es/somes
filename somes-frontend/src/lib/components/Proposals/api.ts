import { getWithRoute } from '$lib/api/api';
import { getParliament, type Parliament } from '$lib/api/parliament';
import type { GovProposalDelegate, HasError, MoodBarometer } from '$lib/types';

export async function gov_proposal_by_path(
	gp: string,
	inr: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<GovProposalDelegate | HasError> {
	return getWithRoute<GovProposalDelegate>(`v1/gov_proposals/${gp}/${inr}`, parliament, fetcher);
}

export async function mood_by_path(
	gp: string,
	inr: string | number,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<MoodBarometer | null | HasError> {
	return getWithRoute<MoodBarometer | null>(`v1/gov_proposals/${gp}/${inr}/mood`, parliament, fetcher);
}
