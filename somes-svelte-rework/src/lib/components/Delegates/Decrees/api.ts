import { justPost } from "$lib/api/api";
import type { HasError } from "$lib/types";
import type { DecreeFilter, DecreesWithMaxPage } from "./types";

export async function decrees_per_page(
	page: number,
	filter: DecreeFilter | null
): Promise<DecreesWithMaxPage | HasError> {
	return justPost(`decrees_per_page?page=${page}`, filter);
}

