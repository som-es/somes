import { describe, expect, it } from 'vitest';
import type { Delegate } from '$lib/types';
import { delegate_seo, served_years } from './delegate';

const delegate = {
	id: 785,
	name: 'Langthaler Monika, Ing.',
	party: 'GRÜNE',
	current_party: 'GRÜNE',
	image_url: null,
	mandates: [
		{
			start_date: '1990-11-05',
			end_date: '1999-10-28',
			name: 'Abgeordnete zum Nationalrat',
			party: 'GRÜNE',
			is_nr: true,
			is_gov_official: false,
			is_ministry: false,
			is_chancellor: false,
			function: 'NR'
		}
	]
} as unknown as Delegate;

describe('delegate_seo', () => {
	it('builds a period-independent title, description and Person JSON-LD', () => {
		const seo = delegate_seo(delegate, 'at');
		expect(seo.title).toBe(
			'Langthaler Monika (GRÜNE) – Abstimmungsverhalten im Nationalrat | Somes'
		);
		expect(seo.description).toBe(
			'Langthaler Monika (GRÜNE): Abgeordnete/r zum Nationalrat (1990–1999). Abstimmungsverhalten, Reden, Zwischenrufe und Interessen auf Somes.'
		);
		expect(seo.jsonLd).toMatchObject({
			'@type': 'Person',
			name: 'Langthaler Monika',
			honorificPrefix: 'Ing.',
			jobTitle: 'Abgeordnete zum Nationalrat',
			url: 'https://somes.at/at/delegates?delegate=785',
			memberOf: { name: 'GRÜNE' },
			sameAs: ['https://www.parlament.gv.at/person/785']
		});
	});

	it('handles ongoing mandates and missing data', () => {
		const single = {
			...delegate,
			mandates: [{ ...delegate.mandates![0], start_date: '2000-01-01', end_date: null }]
		} as Delegate;
		expect(served_years(single, 'at')).toBe('seit 2000');
		expect(served_years({ ...delegate, mandates: null }, 'at')).toBeNull();
	});
});
