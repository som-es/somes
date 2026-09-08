import type { Parliament } from '$lib/api/parliament';
import { loadQuestionDelegate, loadQuestionRecipient } from '$lib/components/Questions/data';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const parliament = params.parliament as Parliament;
	const delegateId = Number(params.delegateId);
	if (!Number.isFinite(delegateId)) return { delegate: null, recipient: null };

	const delegate = await loadQuestionDelegate(fetch, parliament, delegateId);
	const recipient = delegate ? await loadQuestionRecipient(fetch, parliament, delegate) : null;

	return { delegate, recipient };
};
