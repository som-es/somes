import {
	coalition_parties_per_gp,
	parties_per_gp,
	vote_results_by_query_search,
	vote_results_by_search
} from '$lib/api/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders, url }) => {
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}

	const searchParams = url.searchParams;

	if (searchParams.get('page') == null && searchParams.get('sort') == null) {
		searchParams.set('page', '1');
		searchParams.set('sort', 'Desc');
	}

	const queryParams = searchParams.toString();
	const filter = `${queryParams}&is_finished=false`;

	const [voteResults, partiesPerGp, coalitionPartiesPerGp] = await Promise.all([
		vote_results_by_query_search(filter, fetch),
		parties_per_gp(fetch),
		coalition_parties_per_gp(fetch)
	]);

	return {
		voteResults,
		partiesPerGp,
		coalitionPartiesPerGp,
		selectedGp: searchParams.get('legislative_initiative[gp][in][0]')
	};
};
