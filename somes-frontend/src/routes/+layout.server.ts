import { delegate_question_status, isHasError } from '$lib/api/api';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const status = await delegate_question_status(fetch);
	return {
		questionsEnabled: !isHasError(status) && status.enabled
	};
};
