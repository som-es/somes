import { describe, expect, it } from 'vitest';
import {
	absolute,
	route_path,
	static_sitemap_urls,
	to_index_xml,
	to_urlset_xml,
	STATIC_SITEMAP_PATH
} from './sitemap';

describe('route_path', () => {
	it('builds the detail route of every sitemap kind', () => {
		expect(route_path('at', 'decrees', { ris_id: 'BGBLA_2026_II_272' })).toBe(
			'/at/decree/BGBLA_2026_II_272'
		);
		expect(route_path('eu', 'gov_proposals', { gp: '10', inr: 42 })).toBe(
			'/eu/gov_proposal/10/ME/42'
		);
		expect(route_path('at', 'questions', { id: 7 })).toBe('/at/questions/7');
		expect(route_path('eu', 'vote_results', { gp: '10', ityp: 'INI', inr: 12 })).toBe(
			'/eu/vote_result/10/INI/12'
		);
	});
});

describe('static_sitemap_urls', () => {
	const locs = static_sitemap_urls().map((url) => url.loc);

	it('lists the landing page and the parliament pages of both parliaments', () => {
		expect(locs).toContain(`${absolute('/')}`);
		expect(locs).toContain(`${absolute('/at/home')}`);
		expect(locs).toContain(`${absolute('/eu/statistics/absences')}`);
	});

	it('does not list routes without a page', () => {
		expect(locs).not.toContain(`${absolute('/at')}`);
		expect(locs.some((loc) => loc.includes('/user'))).toBe(false);
		expect(locs.some((loc) => loc.includes('/admin'))).toBe(false);
		expect(locs.some((loc) => loc.includes('/questions/ask'))).toBe(false);
	});
});

describe('to_urlset_xml', () => {
	it('escapes the loc and keeps lastmod optional', () => {
		const xml = to_urlset_xml([
			{ loc: 'https://somes.at/a?x=1&y=2', lastmod: '2030-01-01T00:00:00Z' },
			{ loc: 'https://somes.at/b' }
		]);

		expect(xml).toContain('<loc>https://somes.at/a?x=1&amp;y=2</loc>');
		expect(xml).toContain('<lastmod>2030-01-01T00:00:00Z</lastmod>');
		expect(xml).toContain('<url><loc>https://somes.at/b</loc></url>');
	});
});

describe('to_index_xml', () => {
	it('wraps every child in a sitemap node', () => {
		expect(to_index_xml([absolute(STATIC_SITEMAP_PATH)])).toContain(
			'<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><sitemap><loc>https://somes.at/sitemap/static.xml</loc></sitemap></sitemapindex>'
		);
	});
});
