import type { Parliament } from '$lib/api/parliament';
import type { Delegate } from '$lib/types';
import { delegate_canonical_url } from './canonical';

const COUNCIL: Record<Parliament, { title: string; job: string }> = {
	at: { title: 'Abstimmungsverhalten im Nationalrat', job: 'Abgeordnete/r zum Nationalrat' },
	eu: {
		title: 'Abstimmungsverhalten im Europäischen Parlament',
		job: 'Mitglied des Europäischen Parlaments'
	}
};

export function split_delegate_name(name: string): { name: string; honorific: string | null } {
	const [plain, ...rest] = name.split(',');
	return { name: plain.trim(), honorific: rest.join(',').trim() || null };
}

export function delegate_profile_url(parliament: Parliament, id: number): string {
	return parliament === 'at'
		? `https://www.parlament.gv.at/person/${id}`
		: `https://www.europarl.europa.eu/meps/de/${id}`;
}

export function served_years(delegate: Delegate, parliament: Parliament): string | null {
	const mandates = (delegate.mandates ?? []).filter(
		(m) => m.start_date && (parliament !== 'at' || m.is_nr)
	);
	if (mandates.length === 0) return null;
	const first = Math.min(...mandates.map((m) => new Date(m.start_date!).getFullYear()));
	if (mandates.some((m) => !m.end_date)) return `seit ${first}`;
	const last = Math.max(...mandates.map((m) => new Date(m.end_date!).getFullYear()));
	return first === last ? `${first}` : `${first}–${last}`;
}

export function delegate_seo(delegate: Delegate, parliament: Parliament) {
	const council = COUNCIL[parliament];
	const { name, honorific } = split_delegate_name(delegate.name);
	const party = (delegate.current_party || delegate.party)?.trim() || null;
	const nameWithParty = party ? `${name} (${party})` : name;
	const years = served_years(delegate, parliament);
	const jobTitle =
		[...(delegate.mandates ?? [])]
			.filter((m) => m.name)
			.sort((a, b) => (b.start_date ?? '').localeCompare(a.start_date ?? ''))[0]?.name ??
		council.job;

	const jsonLd: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'Person',
		name,
		url: delegate_canonical_url(parliament, delegate.id),
		jobTitle,
		sameAs: [delegate_profile_url(parliament, delegate.id)]
	};
	if (honorific) jsonLd.honorificPrefix = honorific;
	if (party) {
		jsonLd.affiliation = { '@type': 'PoliticalParty', name: party };
		jsonLd.memberOf = { '@type': 'PoliticalParty', name: party };
	}

	return {
		title: `${nameWithParty} – ${council.title} | Somes`,
		description: `${nameWithParty}: ${council.job}${years ? ` (${years})` : ''}. Abstimmungsverhalten, Reden, Zwischenrufe und Interessen auf Somes.`,
		jsonLd
	};
}

export function json_ld_script(data: Record<string, unknown>): string {
	return `<script type="application/ld+json">${JSON.stringify(data).replace(/</g, '\\u003c')}</script>`;
}
