import { delegates_at } from '$lib/api/api';
import type { ArticleLink, Delegate, VoteResult } from '$lib/types';

export function mockArticleLinks(): ArticleLink[] {
	return [
		{
			provider: 'derstandard',
			url: 'https://derstandard.at/3000111223344/Antrag-im-Nationalrat-abgelehnt',
			score: 0.92,
			lastmod: '2026-05-12T14:20:00'
		},
		{
			provider: 'derstandard',
			url: 'https://derstandard.at/3000111223355/Reaktion-der-Koalition',
			score: 0.64,
			lastmod: '2026-05-13T09:05:00'
		},
		{
			provider: 'diepresse',
			url: 'https://www.diepresse.com/6012345/namentliche-abstimmung-im-plenum',
			score: 0.81,
			lastmod: '2026-05-12T18:40:00'
		},
		{
			provider: 'heute',
			url: 'https://www.heute.at/r/9f2c1a3b-0de4-4a11-8c3b-77aa10bb2c01',
			score: 0.55,
			lastmod: '2026-05-12T11:00:00'
		},
		{
			provider: 'kurier',
			url: 'https://kurier.at/politik/inland/abstimmung-ueber-den-antrag/402981234',
			score: 0.77,
			lastmod: '2026-05-12T16:30:00'
		},
		{
			provider: 'oe24',
			url: 'https://www.oesterreich24.at/artikel/abstimmung-nationalrat-2026',
			score: 0.48,
			lastmod: '2026-05-11T20:15:00'
		},
		{
			provider: 'profil',
			url: 'https://www.profil.at/politik/analyse-der-Abstimmung-1234567',
			score: 0.69,
			lastmod: '2026-05-14T08:00:00'
		},
		{
			provider: 'vorarlberger-nachrichten',
			url: 'https://www.vn.at/politik/regionale-reaktion-auf-den-beschluss',
			score: 0.31,
			lastmod: '2026-05-15T07:45:00'
		}
	];
}

function generateDelegate(party: string): Delegate {
	return {
		id: 0,
		name: 'Max Mustermann',
		party: party,
		current_party: party,
		image_url: '',
		constituency: 'Wien',
		council: 'nr',
		seat_row: null,
		seat_col: null,
		gender: 'm',
		is_active: true,
		birthdate: '2000-01-01',
		active_since: new Date(),
		divisions: null,
		active_mandates: [],
		mandates: []
	};
}

export function mockDelegates(): Delegate[] {
	const delegates: Delegate[] = [];
	[
		['SPÖ', 40],
		['FPÖ', 30],
		['NEOS', 15],
		['ÖVP', 71],
		['GRÜNE', 26]
	].forEach(([party, count]) => {
		// @ts-ignore
		for (let i = 0; i < count; i++) {
			// @ts-ignore
			delegates.push(generateDelegate(party));
		}
	});

	return delegates;
}

export function mockDelegatesNoColor(): Delegate[] {
	const delegates: Delegate[] = [];
	[
		['OK', 40],
		['OK', 30],
		['OK', 15],
		['OK', 71],
		['OK', 26]
	].forEach(([party, count]) => {
		// @ts-ignore
		for (let i = 0; i < count; i++) {
			// @ts-ignore
			delegates.push(generateDelegate(party));
		}
	});

	return delegates;
}

export function mockVoteResult(date: string = '2020-03-06'): VoteResult {
	return {
		legislative_initiative: {
			id: '0',
			ityp: '',
			gp: 'XXV',
			title: '',
			description: '',
			emphasis: '',
			accepted: '',
			created_at: date as unknown as Date,
			raw_data_created_at: new Date(),
			raw_data_updated_at: new Date(),
			requires_simple_majority: true,
			voted_by_name: false,
			inr: 0,
			is_emphasis_ai_generated: null,
			plenary_session_id: 0,
			pre_declined_type: ''
		},
		votes: [
			{
				party: 'SPÖ',
				fraction: 40,
				infavor: false,
				legislative_initiatives_id: 0,
				code: 'S'
			},
			{
				party: 'FPÖ',
				fraction: 30,
				infavor: true,
				legislative_initiatives_id: 0,
				code: 'F'
			},
			{
				party: 'NEOS',
				fraction: 15,
				infavor: true,
				legislative_initiatives_id: 0,
				code: 'N'
			},
			{
				party: 'ÖVP',
				fraction: 71,
				infavor: false,
				code: 'V',
				legislative_initiatives_id: 0
			},
			{
				party: 'GRÜNE',
				fraction: 26,
				infavor: true,
				code: 'G',
				legislative_initiatives_id: 0
			}
		],
		speeches: [],
		topics: [],
		named_votes: null,
		documents: [],
		absences: []
	};
}
