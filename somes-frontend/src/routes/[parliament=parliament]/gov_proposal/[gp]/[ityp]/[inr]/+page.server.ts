import { vote_result_by_path } from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import { gov_proposal_by_path } from '$lib/components/Proposals/api';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { or_error } from '$lib/api/load_error';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => {
	const parliament = params.parliament as Parliament;

	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}

	if (params.ityp !== 'ME') {
		redirect(301, `/${parliament}/gov_proposal/${params.gp}/ME/${params.inr}`);
	}

	const govProposal = or_error(
		await gov_proposal_by_path(params.gp, params.inr, fetch, parliament)
	);

	return {
		govProposal
	};
};
