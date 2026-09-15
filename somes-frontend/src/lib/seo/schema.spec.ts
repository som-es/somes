import { describe, expect, it } from 'vitest';
import {
	eventNode,
	eventNodes,
	jsonLd,
	jsonLdScript,
	ORG_ID,
	placeNode,
	siteSchemaNodes,
	toViennaIsoDateTime,
	type SomesEventLike,
	type SchemaNode
} from './schema';

/** Vienna ("Europe/Vienna") wall clock times as stored by the events API. */
function event(overrides: Partial<SomesEventLike> = {}): SomesEventLike {
	return {
		id: 7,
		title: 'Podiumsdiskussion: Demokratie im Gespräch',
		location: 'Haus der Musik, Sonnengasse 1, 3470 Kirchberg am Wagram',
		event_date: '2030-04-12',
		start_time: '19:00',
		description: 'Ein Abend über politische Beteiligung.',
		image: null,
		requires_membership: false,
		requires_registration: false,
		...overrides
	};
}

describe('toViennaIsoDateTime', () => {
	it('adds the standard-time offset in winter', () => {
		expect(toViennaIsoDateTime('2030-01-10', '19:00')).toBe('2030-01-10T19:00:00+01:00');
	});

	it('adds the summer-time offset in summer', () => {
		expect(toViennaIsoDateTime('2030-06-10', '19:00')).toBe('2030-06-10T19:00:00+02:00');
	});

	it('switches offsets on the EU changeover days', () => {
		// 2030-03-31 is the last Sunday in March, 2030-10-27 the last in October.
		expect(toViennaIsoDateTime('2030-03-30', '20:15')).toBe('2030-03-30T20:15:00+01:00');
		expect(toViennaIsoDateTime('2030-03-31', '20:15')).toBe('2030-03-31T20:15:00+02:00');
		expect(toViennaIsoDateTime('2030-10-26', '20:15')).toBe('2030-10-26T20:15:00+02:00');
		expect(toViennaIsoDateTime('2030-10-27', '20:15')).toBe('2030-10-27T20:15:00+01:00');
	});

	it('keeps seconds and assumes midnight when no time is given', () => {
		expect(toViennaIsoDateTime('2030-01-10', '19:00:30')).toBe('2030-01-10T19:00:30+01:00');
		expect(toViennaIsoDateTime('2030-01-10', null)).toBe('2030-01-10T00:00:00+01:00');
	});

	it('rejects dates it cannot interpret', () => {
		expect(toViennaIsoDateTime('', '19:00')).toBeNull();
		expect(toViennaIsoDateTime('12.04.2030', '19:00')).toBeNull();
	});
});

describe('placeNode', () => {
	it('splits an Austrian address out of the free-text location', () => {
		expect(placeNode('Sonnengasse 1, 3470 Kirchberg am Wagram')).toEqual({
			'@type': 'Place',
			name: 'Sonnengasse 1, 3470 Kirchberg am Wagram',
			address: {
				'@type': 'PostalAddress',
				streetAddress: 'Sonnengasse 1',
				postalCode: '3470',
				addressLocality: 'Kirchberg am Wagram',
				addressCountry: 'AT'
			}
		});
	});

	it('keeps venues without an address as name only', () => {
		expect(placeNode('  Café Prückel  ')).toEqual({ '@type': 'Place', name: 'Café Prückel' });
	});
});

describe('eventNode', () => {
	const now = new Date('2030-01-01T12:00:00Z');

	it('marks up an upcoming event', () => {
		expect(eventNode(event(), { now })).toMatchObject({
			'@type': 'Event',
			name: 'Podiumsdiskussion: Demokratie im Gespräch',
			startDate: '2030-04-12T19:00:00+02:00',
			eventStatus: 'https://schema.org/EventScheduled',
			offers: { '@type': 'Offer', price: '0', priceCurrency: 'EUR' },
			organizer: { '@id': ORG_ID },
			location: { '@type': 'Place', name: expect.stringContaining('3470') }
		});
	});

	it('skips events that have already started', () => {
		expect(eventNode(event({ event_date: '2020-04-12' }), { now })).toBeNull();
	});

	it('falls back to the association logo when an event has no image', () => {
		expect(eventNode(event(), { now })?.image).toBe('https://somes.at/somes-logo.png');
		expect(eventNode(event({ image: '/events/podium.jpg' }), { now })?.image).toBe(
			'https://somes.at/events/podium.jpg'
		);
	});
});

describe('eventNodes', () => {
	const now = new Date('2030-01-01T12:00:00Z');

	it('returns upcoming events only, earliest first', () => {
		const nodes = eventNodes(
			[
				event({ id: 1, event_date: '2030-06-01' }),
				event({ id: 2, event_date: '2020-06-01' }),
				event({ id: 3, event_date: '2030-03-01' })
			],
			{ now }
		);

		expect(nodes.map((node) => node['@id'])).toEqual([
			'https://somes.at/#event-3',
			'https://somes.at/#event-1'
		]);
	});
});

describe('jsonLd', () => {
	it('escapes angle brackets so event titles cannot close the script tag', () => {
		const payload = jsonLd([eventNode(event({ title: '</script><script>alert(1)</script>' }))!]);

		expect(payload).not.toContain('</script>');
		expect(JSON.parse(payload)['@graph'][0].name).toBe('</script><script>alert(1)</script>');
	});

	it('wraps the nodes in a schema.org @graph', () => {
		const parsed = JSON.parse(jsonLd(siteSchemaNodes())) as { '@graph': SchemaNode[] } & SchemaNode;

		expect(parsed['@context']).toBe('https://schema.org');
		expect(parsed['@graph'].map((node) => node['@type'])).toEqual(['Organization', 'WebSite']);
	});

	it('renders one JSON-LD tag and none at all for an empty graph', () => {
		const script = jsonLdScript(siteSchemaNodes());

		expect(script.startsWith('<script type="application/ld+json">')).toBe(true);
		expect(script.endsWith('</script>')).toBe(true);
		expect(jsonLdScript([])).toBe('');
	});
});
