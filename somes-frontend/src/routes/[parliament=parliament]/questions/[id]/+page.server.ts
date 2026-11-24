import type { Parliament } from '$lib/api/parliament';
import { loadFullQuestionDelegate, loadQuestionEntry } from '$lib/components/Questions/data';
import type { PageServerLoad } from './$types';
import { delegate_question_by_id } from '$lib/api/api';
import { or_error } from '$lib/api/load_error';

export const load: PageServerLoad = async ({ fetch, params, setHeaders }) => {
	const parliament = params.parliament as Parliament;
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}

	const entry = await loadQuestionEntry(fetch, parliament, params.id);
	if (!entry) or_error(await delegate_question_by_id(params.id, fetch, parliament));
	const fullDelegate = entry?.delegate
		? await loadFullQuestionDelegate(fetch, parliament, entry.delegate.id)
		: null;

	return { entry, fullDelegate };
};
