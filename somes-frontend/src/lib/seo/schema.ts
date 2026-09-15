/**
 * schema.org structured data (JSON-LD) builders.
 *
 * Google uses these to build rich results:
 * - Organization: https://developers.google.com/search/docs/appearance/structured-data/organization
 * - Event: https://developers.google.com/search/docs/appearance/structured-data/event
 *
 * The helpers here are framework agnostic and pure (they take a `now` so they
 * can be tested); `JsonLd.svelte` renders them into `<svelte:head>`. Both the
 * root layout and the landing page are SSR'ed, so the markup is part of the
 * initial HTML that the crawler sees.
 */

/** Any schema.org node, e.g. `{ '@type': 'Place', name: '...' }`. */
export type SchemaNode = Record<string, unknown>;

/** Subset of `SomesEvent` (`src/routes/types.ts`) that we can mark up. */
export interface SomesEventLike {
	id: number | null;
	title: string;
	location: string;
	/** ISO date, `YYYY-MM-DD`. */
	event_date: string;
	/** Wall clock time in Vienna, `HH:MM` or `HH:MM:SS`. */
	start_time: string;
	description: string;
	image: string | null;
	requires_membership: boolean;
	requires_registration: boolean;
}

/**
 * Canonical origin of the site.
 *
 * Identity properties (`@id`, `url`, `logo`) deliberately do *not* follow the
 * request URL: the association must always be described as the same entity,
 * not as a second one living on the preview/staging host.
 */
export const SITE_URL = 'https://somes.at';

export const ORG_ID = `${SITE_URL}/#organization`;
export const WEBSITE_ID = `${SITE_URL}/#website`;

/**
 * Logo with a stable path (served from `static/`), unlike the hashed build
 * asset. Google needs a crawlable, non-changing logo URL (min. 112x112 px).
 */
export const LOGO_URL = `${SITE_URL}/somes-logo.png`;
const LOGO_WIDTH = 838;
const LOGO_HEIGHT = 748;

/** Legal name from the Impressum (§ 25 Mediengesetz). */
export const ORG_NAME = 'somes - Verein für Demokratie und politische Transparenz';

const ORG_EMAIL = 'somes.austria@gmail.com';
const ORG_ZVR = '1443766724';

const ORG_DESCRIPTION =
	'Parteiübergreifend machen wir Demokratie transparent, verständlich und zugänglich: ' +
	'die Plattform somes.at bereitet Abstimmungen, Reden, Ministerialentwürfe und Verordnungen ' +
	'des Nationalrats und des EU-Parlaments mit KI auf. Als Verein veranstalten wir ' +
	'Podiumsdiskussionen und Workshops über Politik und Demokratie.';

const ORG_SAME_AS = [
	'https://www.instagram.com/somes.at',
	'https://www.linkedin.com/company/somes-at',
	'https://github.com/som-es',
	'https://www.netidee.at/somes'
];

/** Events live in the `#events` section of the landing page — there is no per-event URL. */
const EVENTS_ANCHOR = `${SITE_URL}/#events`;

/** All events are hosted by the association itself, in person, at `event.location`. */
function organizationNode(): SchemaNode {
	return {
		'@type': 'Organization',
		'@id': ORG_ID,
		name: ORG_NAME,
		legalName: ORG_NAME,
		alternateName: ['somes', 'somes.at', 'Association for Democracy and Political Transparency'],
		url: `${SITE_URL}/`,
		description: ORG_DESCRIPTION,
		email: ORG_EMAIL,
		logo: {
			'@type': 'ImageObject',
			'@id': `${SITE_URL}/#logo`,
			url: LOGO_URL,
			contentUrl: LOGO_URL,
			width: LOGO_WIDTH,
			height: LOGO_HEIGHT,
			caption: ORG_NAME
		},
		address: {
			'@type': 'PostalAddress',
			streetAddress: 'Sonnengasse 1',
			postalCode: '3470',
			addressLocality: 'Kirchberg am Wagram',
			addressRegion: 'Niederösterreich',
			addressCountry: 'AT'
		},
		contactPoint: {
			'@type': 'ContactPoint',
			contactType: 'customer support',
			email: ORG_EMAIL,
			availableLanguage: ['German', 'English']
		},
		// Vereinsregister number (ZVR-Zahl) — helps Google disambiguate the org.
		identifier: {
			'@type': 'PropertyValue',
			propertyID: 'ZVR-Zahl',
			value: ORG_ZVR
		},
		funder: { '@type': 'Organization', name: 'Netidee (FFG)', url: 'https://www.netidee.at' },
		knowsLanguage: ['de', 'en'],
		areaServed: { '@type': 'Country', name: 'Österreich' },
		sameAs: ORG_SAME_AS
	};
}

function webSiteNode(): SchemaNode {
	return {
		'@type': 'WebSite',
		'@id': WEBSITE_ID,
		url: `${SITE_URL}/`,
		name: 'somes.at',
		alternateName: 'somes',
		description: ORG_DESCRIPTION,
		inLanguage: ['de', 'en'],
		publisher: { '@id': ORG_ID }
	};
}

/** Organization + WebSite, rendered site wide by the root layout. */
export function siteSchemaNodes(): SchemaNode[] {
	return [organizationNode(), webSiteNode()];
}

/**
 * Austrian addresses are `PLZ Ort`, so the free-text location can usually be
 * split into a postal address. `Place.name` always carries the full string —
 * Google requires the `name`, the `address` is a bonus.
 */
export function placeNode(location: string): SchemaNode {
	const name = location.trim();
	const place: SchemaNode = { '@type': 'Place', name };

	// "Sonnengasse 1, 3470 Kirchberg am Wagram" -> street "Sonnengasse 1",
	// plz "3470", city "Kirchberg am Wagram".
	const match = /(?<!\d)\d{4}(?!\d)\s+[^,\d]/.exec(name);
	if (match) {
		const plz = name.slice(match.index, match.index + 4);
		const street = name
			.slice(0, match.index)
			.replace(/[,\s]+$/, '')
			.trim();
		const address: SchemaNode = {
			'@type': 'PostalAddress',
			postalCode: plz,
			addressLocality: name
				.slice(match.index + plz.length)
				.split(',')[0]
				.trim(),
			addressCountry: 'AT'
		};
		if (street) address.streetAddress = street;
		place.address = address;
	}

	return place;
}

const DATE_RE = /^(\d{4})-(\d{2})-(\d{2})$/;
const TIME_RE = /^(\d{1,2}):(\d{2})(?::(\d{2}))?/;

const pad = (n: number) => String(n).padStart(2, '0');

/**
 * Start of the EU summer time period (last Sunday of March, 01:00 UTC) and
 * its end (last Sunday of October, 01:00 UTC), as UTC timestamps.
 */
function summerTimeBounds(utcMs: number): [number, number] {
	const year = new Date(utcMs).getUTCFullYear();
	const start = (month: number) => {
		const lastDay = new Date(Date.UTC(year, month + 1, 0)).getUTCDate();
		const dayOfWeek = new Date(Date.UTC(year, month, lastDay)).getUTCDay();
		return Date.UTC(year, month, lastDay - dayOfWeek, 1);
	};
	return [start(2), start(9)];
}

/** Offset (minutes) of Europe/Vienna at the given instant. */
function viennaOffsetMs(utcMs: number): number {
	const [start, end] = summerTimeBounds(utcMs);
	return (utcMs >= start && utcMs < end ? 120 : 60) * 60_000;
}

/**
 * `YYYY-MM-DD` + `HH:MM` are Vienna wall clock times, which JSON-LD consumers
 * cannot interpret without a zone. Returns a full ISO timestamp with offset,
 * e.g. `2026-01-10T19:00:00+01:00`, or null if the date is unusable.
 *
 * The DST rule is spelled out instead of using `Intl` so the result does not
 * depend on the ICU data bundled with the server runtime.
 */
export function toViennaIsoDateTime(date: string, time?: string | null): string | null {
	const day = DATE_RE.exec(date?.trim() ?? '');
	if (!day) return null;

	const [, year, month, dayOfMonth] = day.map(Number);
	const clock = TIME_RE.exec(time?.trim() ?? '');
	const [hours, minutes, seconds] = clock
		? [Number(clock[1]), Number(clock[2]), Number(clock[3] ?? '0')]
		: [0, 0, 0];
	if (hours > 23 || minutes > 59 || seconds > 59) return null;

	// Wall clock -> instant: assume standard time, then correct if that instant
	// falls into summer time (i.e. the wall clock reads summer time).
	const wallClock = Date.UTC(year, month - 1, dayOfMonth, hours, minutes, seconds);
	const offsetMs = viennaOffsetMs(wallClock - viennaOffsetMs(wallClock));
	const zoneSign = offsetMs >= 0 ? '+' : '-';
	const zoneHours = pad(Math.floor(Math.abs(offsetMs) / 3_600_000));
	const zoneMinutes = pad((Math.abs(offsetMs) % 3_600_000) / 60_000);

	return `${date}T${pad(hours)}:${pad(minutes)}:${pad(seconds)}${zoneSign}${zoneHours}:${zoneMinutes}`;
}

/**
 * `Event` node for one of our own events (Podiumsdiskussionen etc.), or null
 * when it must not be marked up: Google only shows events that have not
 * started yet, so past events are skipped instead of marked up.
 */
export function eventNode(
	event: SomesEventLike,
	{ now = new Date() }: { now?: Date } = {}
): SchemaNode | null {
	const startDate = toViennaIsoDateTime(event.event_date, event.start_time);
	if (!startDate) return null;

	const start = new Date(startDate);
	if (Number.isNaN(start.getTime()) || start.getTime() < now.getTime()) return null;

	const node: SchemaNode = {
		'@type': 'Event',
		'@id': `${SITE_URL}/#event-${event.id ?? `${event.event_date}-${slug(event.title)}`}`,
		name: event.title,
		description: event.description,
		url: EVENTS_ANCHOR,
		startDate,
		// Google wants an event status; everything we list is on schedule.
		eventStatus: 'https://schema.org/EventScheduled',
		location: placeNode(event.location),
		// Our events are free to attend (members-only events are free too).
		offers: {
			'@type': 'Offer',
			url: EVENTS_ANCHOR,
			price: '0',
			priceCurrency: 'EUR',
			availability: 'https://schema.org/InStock'
		},
		organizer: { '@id': ORG_ID },
		image: event.image ? absoluteUrl(event.image) : LOGO_URL
	};

	if (event.requires_membership) {
		node.audience = { '@type': 'Audience', audienceType: 'Mitglieder von somes' };
	}

	return node;
}

/** `Event` nodes for all upcoming events, oldest first. */
export function eventNodes(
	events: SomesEventLike[],
	{ now = new Date() }: { now?: Date } = {}
): SchemaNode[] {
	return events
		.map((event) => eventNode(event, { now }))
		.filter((node): node is SchemaNode => node !== null)
		.sort((a, b) => String(a.startDate).localeCompare(String(b.startDate)));
}

function slug(text: string): string {
	return text
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/^-|-$/g, '')
		.slice(0, 60);
}

/** Backend assets may come back as root-relative paths; schema URLs must be absolute. */
function absoluteUrl(url: string): string {
	return url.startsWith('http') ? url : `${SITE_URL}${url.startsWith('/') ? '' : '/'}${url}`;
}

/**
 * Serialize nodes to the content of a `<script type="application/ld+json">`.
 *
 * `<`, `>` and `&` never occur in JSON outside string values, so escaping them
 * as unicode escapes both keeps the JSON valid and makes it impossible for a
 * user-supplied event title to close the script tag.
 */
export function jsonLd(nodes: SchemaNode[]): string {
	const graph = { '@context': 'https://schema.org', '@graph': nodes };
	return JSON.stringify(graph, null, 2)
		.replace(/</g, '\\u003c')
		.replace(/>/g, '\\u003e')
		.replace(/&/g, '\\u0026')
		.replace(/\u2028/g, '\\u2028')
		.replace(/\u2029/g, '\\u2029');
}

/**
 * Ready-to-insert `<script type="application/ld+json">` tag, empty when there
 * is nothing to mark up (an empty JSON-LD tag is a rich results error).
 *
 * Lives here because a `.svelte` file cannot spell the closing script tag: the
 * Svelte parser would end its own `<script>` block there.
 */
export function jsonLdScript(nodes: SchemaNode[]): string {
	if (nodes.length === 0) return '';
	return `<script type="application/ld+json">${jsonLd(nodes)}</script>`;
}
