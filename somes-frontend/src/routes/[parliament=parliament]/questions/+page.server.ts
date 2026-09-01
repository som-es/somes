import type { Parliament } from '$lib/api/parliament';
import { loadQuestionEntries } from '$lib/components/Questions/data';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => {
	const parliament = params.parliament as Parliament;
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}

	return { entries: await loadQuestionEntries(fetch, parliament) };
};
