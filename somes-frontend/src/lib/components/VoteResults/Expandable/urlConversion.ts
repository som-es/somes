import { page } from '$app/state';
import { plink } from '$lib/api/parliament';
import type { VoteResultFilter } from '$lib/types';

export function convertVoteResultFilterToUrl(
	filter: VoteResultFilter | null,
	searchValue: string,
	currentUrl: URL | undefined,
	isFinished: boolean = true,
	sort: 'Desc' | 'Asc' | 'relevance' = 'relevance'
): URL {
	const nextUrl = currentUrl
		? currentUrl
		: new URL(
				isFinished ? plink('/history/votes') : plink('/history/unfinished_votes'),
				page.url.origin
			);

	nextUrl.search = '';

	const pageValue = 1;
	nextUrl.searchParams.set('page', pageValue.toString());
	if (filter === null) {
		nextUrl.searchParams.set('sort', 'Desc');
		return nextUrl;
	}

	nextUrl.searchParams.set('page', (filter.page ?? pageValue).toString());

	if (filter.is_named_vote !== null) {
		nextUrl.searchParams.set(
			'legislative_initiative[voted_by_name][eq]',
			filter.is_named_vote.toString()
		);
	}
	if (filter.accepted !== null) {
		nextUrl.searchParams.set('legislative_initiative[accepted][eq]', filter.accepted);
	}
	if (filter.vote_type.length > 0) {
		nextUrl.searchParams.set('legislative_initiative[voting][in][0]', filter.vote_type[0]);
	}

	if (filter.gps.length > 0) {
		nextUrl.searchParams.set('legislative_initiative[gp][in][0]', filter.gps[0]);
	}

	if (filter.simple_majority !== null) {
		nextUrl.searchParams.set(
			'legislative_initiative[requires_simple_majority][eq]',
			filter.simple_majority.toString()
		);
	}

	if (filter.is_urgent !== null) {
		nextUrl.searchParams.set('legislative_initiative[is_urgent][eq]', filter.is_urgent.toString());
	}

	filter.party_votes?.forEach((partyVotes, i) => {
		nextUrl.searchParams.set(`party_votes[${i}][infavor]`, partyVotes.infavor.toString());
		nextUrl.searchParams.set(`party_votes[${i}][party]`, partyVotes.party);
	});

	if (filter.date_from) {
		nextUrl.searchParams.set('date_from', filter.date_from);
	}
	if (filter.date_to) {
		nextUrl.searchParams.set('date_to', filter.date_to);
	}

	if (filter.is_from_governemnt !== null) {
		nextUrl.searchParams.set(
			`legislative_initiative[doktyp][${filter.is_from_governemnt ? 'in' : 'nin'}][0]`,
			'RV'
		);
	}

	// enforce with frontend => add user sorting
	if (searchValue.length === 0 || sort === 'Desc') {
		nextUrl.searchParams.set('sort', 'Desc');
	} else if (sort === 'Asc') {
		nextUrl.searchParams.set('sort', 'Asc');
	}
	// else relevance: no sort param, backend uses relevance ranking

	nextUrl.searchParams.set('search', searchValue);

	filter.topics?.forEach((topic, i) => {
		nextUrl.searchParams.set(`filters[0][or][0][eurovoc_topics][${i}][topic][cn]`, topic);
		nextUrl.searchParams.set(`filters[0][or][1][ai_summary][full_summary][topics][in][${i}]`, topic);
	});

	filter.issuer_parties?.forEach((party, i) => {
		nextUrl.searchParams.set(`meilisearch_helper[issuer_parties][in][${i}]`, party);
	});

	return nextUrl;
}
