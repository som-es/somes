import { getWithRoute, justPost } from '$lib/api/api';
import { getParliament, plink, type Parliament } from '$lib/api/parliament';
import type { HasError } from '$lib/types';
import type { Decree, DecreeFilter, DecreesWithMaxPage } from './types';

export function createDecreePath(ris_id: string): string {
	return plink(`/decree/${ris_id}`);
}

export async function decrees_per_page(
	page: number,
	filter: DecreeFilter | null
): Promise<DecreesWithMaxPage | HasError> {
	return justPost(`decrees_per_page?page=${page}`, filter);
}

export async function decrees_by_search(
	query: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<DecreesWithMaxPage | HasError> {
	return getWithRoute<DecreesWithMaxPage>(`v1/decrees/search?${query}`, parliament, fetcher);
}

export async function decree_by_ris_id(
	ris_id: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Decree | HasError> {
	return getWithRoute<Decree>(`v1/decrees/ris_id/${ris_id}`, parliament, fetcher);
}
