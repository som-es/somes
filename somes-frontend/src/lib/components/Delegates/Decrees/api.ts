import { resolve } from '$app/paths';
import { getWithRoute, justPost } from '$lib/api/api';
import type { HasError } from '$lib/types';
import type { Decree, DecreeFilter, DecreesWithMaxPage } from './types';

export function createDecreePath(ris_id: string): string {
	return resolve("/decree/[id]", { id: ris_id });
}

export async function decrees_per_page(
	page: number,
	filter: DecreeFilter | null
): Promise<DecreesWithMaxPage | HasError> {
	return justPost(`decrees_per_page?page=${page}`, filter);
}

export async function decrees_by_search(
	page: number,
	filter: DecreeFilter | null,
	search: string
): Promise<DecreesWithMaxPage | HasError> {
	return justPost(`v1/decrees/search?page=${page}&search=${search}`, filter);
}

export async function decree_by_ris_id(ris_id: string, fetcher: typeof fetch = fetch
): Promise<Decree | HasError> {
	return getWithRoute<Decree>(`v1/decrees/ris_id/${ris_id}`, "at/", fetcher);
}
