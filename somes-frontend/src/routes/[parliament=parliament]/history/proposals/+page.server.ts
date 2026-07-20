import { departments_per_gp, gov_proposals_by_search } from '$lib/api/api';
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

	const queryParams = searchParams.toString();
	const filter = `${queryParams}`;
	const govProposals = await gov_proposals_by_search(filter, fetch, parliament);
	const departmentsPerGp = await departments_per_gp(fetch, parliament);
	return {
		govProposals,
		selectedGp: searchParams.get('gov_proposal[ministrial_proposal][gp][in][0]'),
		departmentsPerGp
	};
};
