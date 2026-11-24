import { describe, expect, it } from 'vitest';
import { canonical_url } from './canonical';

const url = (path: string) => new URL(`http://localhost:5173${path}`);

describe('canonical_url', () => {
	it('keeps only the delegate id on delegate pages', () => {
		expect(
			canonical_url(
				url('/at/delegates?delegate=785&date=1997-03-20&gp=XX'),
				'/[parliament=parliament]/delegates'
			)
		).toBe('https://somes.at/at/delegates?delegate=785');
		expect(
			canonical_url(
				url('/eu/delegates?gp=9&delegate=12&date=2020-01-01'),
				'/[parliament=parliament]/delegates'
			)
		).toBe('https://somes.at/eu/delegates?delegate=12');
	});

	it('strips query parameters that do not define the content', () => {
		expect(
			canonical_url(
				url('/at/vote_result/XXVII/A/1?search=x&party=SPÖ'),
				'/[parliament=parliament]/vote_result/[gp]/[ityp]/[inr]'
			)
		).toBe('https://somes.at/at/vote_result/XXVII/A/1');
		expect(canonical_url(url('/?utm_source=x'), '/')).toBe('https://somes.at/');
	});

	it('keeps pagination on lists but drops page 1 and filters', () => {
		const route = '/[parliament=parliament]/history/votes';
		expect(canonical_url(url('/at/history/votes?page=3&sort=asc'), route)).toBe(
			'https://somes.at/at/history/votes?page=3'
		);
		expect(canonical_url(url('/at/history/votes?page=1'), route)).toBe(
			'https://somes.at/at/history/votes'
		);
	});

	it('emits nothing for private pages, errors and unknown routes', () => {
		expect(canonical_url(url('/user'), '/user')).toBeNull();
		expect(canonical_url(url('/at/nope'), null)).toBeNull();
		expect(canonical_url(url('/at/home'), '/[parliament=parliament]/home', 500)).toBeNull();
	});
});
