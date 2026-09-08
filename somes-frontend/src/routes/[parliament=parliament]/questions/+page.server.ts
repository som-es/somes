import type { Parliament } from '$lib/api/parliament';
import { searchQuestionEntries } from '$lib/components/Questions/data';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, setHeaders, url }) => {
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

	return { result: await searchQuestionEntries(fetch, parliament, searchParams.toString()) };
};
