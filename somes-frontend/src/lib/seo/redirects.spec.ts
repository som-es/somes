import { describe, expect, it } from 'vitest';
import { redirect_location } from './redirects';

const at = (path: string) => new URL(`https://somes.at${path}`);

describe('redirect_location', () => {
	it('moves un-prefixed AT routes under /at and keeps the query', () => {
		expect(redirect_location(at('/gov_proposal/XXVII/ME/208'), 'somes.at')).toBe(
			'/at/gov_proposal/XXVII/ME/208'
		);
		expect(redirect_location(at('/decree/BGBLA_2024_II_307'), 'somes.at')).toBe(
			'/at/decree/BGBLA_2024_II_307'
		);
		expect(redirect_location(at('/delegates?delegate=785&gp=XX'), 'somes.at')).toBe(
			'/at/delegates?delegate=785&gp=XX'
		);
		expect(redirect_location(at('/statistics/call_to_orders'), 'somes.at')).toBe(
			'/at/statistics/call_to_orders'
		);
		expect(redirect_location(at('/home'), 'somes.at')).toBe('/at/home');
	});

	it('leaves site-wide, prefixed and EU routes alone', () => {
		for (const path of [
			'/',
			'/orientation',
			'/impressum',
			'/datenschutz',
			'/at/delegates?delegate=785',
			'/eu/vote_result/10/INI/12',
			'/sitemap-index.xml',
			'/homepage'
		]) {
			expect(redirect_location(at(path), 'somes.at')).toBeNull();
		}
	});

	it('sends www to the bare domain in one hop', () => {
		expect(redirect_location(at('/at/home?x=1'), 'www.somes.at')).toBe(
			'https://somes.at/at/home?x=1'
		);
		expect(redirect_location(at('/decree/A'), 'WWW.somes.at')).toBe('https://somes.at/at/decree/A');
	});
});
