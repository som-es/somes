import type { PageServerLoad } from './$types';
import { getWithRoute } from '$lib/api/api';

export const load: PageServerLoad = async ({ fetch }) => {
	// Backend route is under /api/at/orientation_questions
	const questions = await getWithRoute('orientation_questions', 'at', fetch);
	return { questions };
};
