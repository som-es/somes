import { describe, expect, it } from 'vitest';
import { breadcrumb_json_ld } from './breadcrumbs';

describe('breadcrumb_json_ld', () => {
	it('builds a breadcrumb list with absolute urls', () => {
		expect(
			breadcrumb_json_ld([
				{ name: 'Abstimmungen', path: '/at/history/votes' },
				{ name: 'Novelle zum Parteiengesetz' }
			])
		).toEqual({
			'@context': 'https://schema.org',
			'@type': 'BreadcrumbList',
			itemListElement: [
				{
					'@type': 'ListItem',
					position: 1,
					name: 'Abstimmungen',
					item: 'https://somes.at/at/history/votes'
				},
				{ '@type': 'ListItem', position: 2, name: 'Novelle zum Parteiengesetz' }
			]
		});
	});
});
