import { delegates_at } from '$lib/api';
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
		divisions: null
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

export function mockVoteResult(): VoteResult {
	return {
		legislative_initiative: {
			id: '0',
			ityp: '',
			gp: 'XXV',
			title: '',
			description: '',
			emphasis: '',
			accepted: '',
			created_at: '2020-03-06' as unknown as Date,
			appeared_at: new Date(),
			updated_at: new Date(),
			requires_simple_majority: true,
			voted_by_name: false
		},
		votes: [
			{
				party: 'SPÖ',
				fraction: 40,
				infavor: false,
				legislative_initiatives_id: 0
			},
			{
				party: 'FPÖ',
				fraction: 30,
				infavor: true,
				legislative_initiatives_id: 0
			},
			{
				party: 'NEOS',
				fraction: 15,
				infavor: true,
				legislative_initiatives_id: 0
			},
			{
				party: 'ÖVP',
				fraction: 71,
				infavor: false,
				legislative_initiatives_id: 0
			},
			{
				party: 'GRÜNE',
				fraction: 26,
				infavor: true,
				legislative_initiatives_id: 0
			}
		],
		speeches: [],
		topics: [],
		named_votes: null
	};
}
