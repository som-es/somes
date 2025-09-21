import { getWithRoute, justPost } from "$lib/api/api";
import type { HasError } from "$lib/types";
import type { Decree, DecreeFilter, DecreesWithMaxPage } from "./types";

export async function decrees_per_page(
	page: number,
	filter: DecreeFilter | null
): Promise<DecreesWithMaxPage | HasError> {
	return justPost(`decrees_per_page?page=${page}`, filter);
}

export async function decree_by_ris_id(ris_id: string): Promise<Decree | HasError> {
	return getWithRoute<Decree>(`decree?ris_id=${ris_id}`);
}
