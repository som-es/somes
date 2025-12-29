import { delegates_at } from '$lib/api/api';
import type { Delegate, VoteResult } from '$lib/types';

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
				code: "S"
			},
			{
				party: 'FPÖ',
				fraction: 30,
				infavor: true,
				legislative_initiatives_id: 0,
				code: "F"
			},
			{
				party: 'NEOS',
				fraction: 15,
				infavor: true,
				legislative_initiatives_id: 0,
				code: "N",
			},
			{
				party: 'ÖVP',
				fraction: 71,
				infavor: false,
				code: "V",
				legislative_initiatives_id: 0
			},
			{
				party: 'GRÜNE',
				fraction: 26,
				infavor: true,
				code: "G",
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
