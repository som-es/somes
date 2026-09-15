import { getWithRoute, isHasError } from './api';
import type { Parliament } from './parliament';
import type { HasError } from '../types';

export const ENTRIES_PER_PAGE = 1000;

export type SitemapKind = 'decrees' | 'gov_proposals' | 'questions' | 'vote_results';

export interface SitemapRoute {
	gp?: string;
	ityp?: string;
	inr?: number;
	ris_id?: string;
	id?: number;
	lastmod?: string;
}

export interface SitemapEntries<T> {
	entries: T[];
	entry_count: number;
	max_page: number;
}

export interface SitemapGpCount {
	gp: string;
	entry_count: number;
}

export interface SitemapSummary {
	decrees: number;
	gov_proposals: number;
	questions: number;
	vote_results: SitemapGpCount[];
}

export async function sitemap_summary(
	parliament: Parliament,
	fetcher: typeof fetch
): Promise<SitemapSummary | HasError> {
	return getWithRoute<SitemapSummary>('v1/sitemap/summary', parliament, fetcher);
}

async function sitemap_page(
	kind: SitemapKind,
	parliament: Parliament,
	fetcher: typeof fetch,
	page: number,
	gp?: string
): Promise<SitemapEntries<SitemapRoute> | HasError> {
	const query = new URLSearchParams({
		page: page.toString(),
		entries_per_page: ENTRIES_PER_PAGE.toString()
	});
	if (gp) query.set('gp', gp);
	return getWithRoute<SitemapEntries<SitemapRoute>>(
		`v1/sitemap/${kind}?${query}`,
		parliament,
		fetcher
	);
}

export async function sitemap_routes(
	kind: SitemapKind,
	parliament: Parliament,
	fetcher: typeof fetch,
	gp?: string
): Promise<SitemapRoute[]> {
	const routes: SitemapRoute[] = [];

	for (let page = 1; page <= 100; page++) {
		const result = await sitemap_page(kind, parliament, fetcher, page, gp);
		if (isHasError(result)) return routes;
		routes.push(...result.entries);
		if (page >= result.max_page) return routes;
	}

	return routes;
}
