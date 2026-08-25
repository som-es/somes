import { errorToNull } from '$lib/api/api';
import type { PageServerLoad } from './$types';
import { searchDelegates, type SearchFilter } from './searchDelegates';

// ssr delegate search modal
export const load: PageServerLoad = async ({ url, parent }) => {
	const filter: SearchFilter = {
		search: url.searchParams.get('search') ?? '',
		parties: url.searchParams.getAll('party'),
		vote: url.searchParams.get('vote'),
		countries: url.searchParams.getAll('country')
	};

	const { voteResult, delegates } = await parent();

	return {
		filter,
		searchResults: searchDelegates(errorToNull(voteResult), delegates ?? [], filter)
	};
};
