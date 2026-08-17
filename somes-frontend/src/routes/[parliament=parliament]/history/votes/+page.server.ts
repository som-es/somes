import {
	coalition_parties_per_gp,
	parties_per_gp,
	vote_results_by_query_search
} from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, setHeaders, url, params }) => {
	const parliament = params.parliament as Parliament;
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
	const topics = url.searchParams.getAll('topics');

	const queryParams = searchParams.toString();
	const filter = `${queryParams}&is_finished=true`;

	const [voteResults, partiesPerGp, coalitionPartiesPerGp] = await Promise.all([
		vote_results_by_query_search(filter, fetch, parliament),
		parties_per_gp(fetch, parliament),
		coalition_parties_per_gp(fetch, parliament)
	]);

	return {
		voteResults,
		partiesPerGp,
		coalitionPartiesPerGp,
		selectedGp: searchParams.get('legislative_initiative[gp][in][0]')
	};
};
