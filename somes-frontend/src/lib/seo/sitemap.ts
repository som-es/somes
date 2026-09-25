import { PARLIAMENTS, type Parliament } from '$lib/api/parliament';
import type { SitemapKind, SitemapRoute } from '$lib/api/sitemap';

export const SITE_URL = 'https://somes.at';

export interface SitemapUrl {
	loc: string;
	lastmod?: string;
}

export const STATIC_SITEMAP_PATH = '/sitemap/static.xml';

export const SITEMAP_CHILD_KINDS = [
	'static',
	'decrees',
	'gov_proposals',
	'questions',
	'vote_results'
] as const;

export type SitemapChildKind = (typeof SITEMAP_CHILD_KINDS)[number];

const STATIC_PATHS = ['/', '/impressum', '/datenschutz', '/orientation'];

const PARLIAMENT_PATHS = [
	'/home',
	'/delegates',
	'/questions',
	'/statistics',
	'/statistics/absences',
	'/statistics/activity',
	'/statistics/complexity',
	'/statistics/age',
	'/statistics/call_to_orders',
	'/statistics/orientation',
	'/statistics/speech_time',
	'/statistics/total_speeches',
	'/history',
	'/history/decrees',
	'/history/proposals',
	'/history/unfinished_votes',
	'/history/votes'
];

export function absolute(path: string): string {
	return `${SITE_URL}${path}`;
}

export function sitemap_child_path(kind: SitemapKind, parliament: Parliament, gp?: string): string {
	const query = new URLSearchParams({ parliament });
	if (gp) query.set('gp', gp);
	return `/sitemap/${kind}.xml?${query}`;
}

export function route_path(parliament: Parliament, kind: SitemapKind, route: SitemapRoute): string {
	switch (kind) {
		case 'decrees':
			return `/${parliament}/decree/${encodeURIComponent(route.ris_id ?? '')}`;
		case 'gov_proposals':
			return `/${parliament}/gov_proposal/${route.gp}/ME/${route.inr}`;
		case 'questions':
			return `/${parliament}/questions/${route.id}`;
		case 'vote_results':
			return `/${parliament}/vote_result/${route.gp}/${route.ityp}/${route.inr}`;
	}
}

export function static_sitemap_urls(): SitemapUrl[] {
	const paths = [
		...STATIC_PATHS,
		...PARLIAMENTS.flatMap((parliament) => PARLIAMENT_PATHS.map((path) => `/${parliament}${path}`))
	];

	return paths.map((path) => ({ loc: absolute(path) }));
}

export function to_urlset_xml(urls: SitemapUrl[]): string {
	const entries = urls
		.map(
			(url) =>
				`<url><loc>${escape_xml(url.loc)}</loc>${
					url.lastmod ? `<lastmod>${escape_xml(url.lastmod)}</lastmod>` : ''
				}</url>`
		)
		.join('');

	return `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${entries}</urlset>`;
}

export function to_index_xml(locs: string[]): string {
	const entries = locs.map((loc) => `<sitemap><loc>${escape_xml(loc)}</loc></sitemap>`).join('');

	return `<?xml version="1.0" encoding="UTF-8"?>\n<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${entries}</sitemapindex>`;
}

export function xml_response(body: string): Response {
	return new Response(body, {
		headers: {
			'content-type': 'application/xml; charset=utf-8',
			'cache-control': 'public, max-age=3600, stale-while-revalidate=86400'
		}
	});
}

function escape_xml(value: string): string {
	return value
		.replaceAll('&', '&amp;')
		.replaceAll('<', '&lt;')
		.replaceAll('>', '&gt;')
		.replaceAll('"', '&quot;')
		.replaceAll("'", '&apos;');
}
