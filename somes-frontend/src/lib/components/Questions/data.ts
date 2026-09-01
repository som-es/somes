import {
	all_delegate_questions,
	delegate_by_id,
	delegate_questions,
	errorToNull
} from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type { Delegate } from '$lib/types';
import { mockQuestionEntries } from './mock';
import {
	parseQuestionSlug,
	questionSlug,
	type DelegateQuestionView,
	type QuestionDelegate
} from './types';

// The delegate questions API exists on this branch but is not deployed yet.
// Until it is live we serve mock data in the exact shape the API will return.
// TODO: flip to true (and delete mock.ts) once the API is live.
const API_LIVE: boolean = false;

function toQuestionDelegate(delegate: Delegate | null): QuestionDelegate | null {
	if (delegate === null) return null;
	return {
		id: delegate.id,
		name: delegate.name,
		party: delegate.current_party ?? delegate.party ?? null
	};
}

export async function loadQuestionEntries(
	fetcher: typeof fetch,
	parliament: Parliament
): Promise<DelegateQuestionView[]> {
	if (!API_LIVE) return mockQuestionEntries(parliament);

	const questions = errorToNull(await all_delegate_questions(fetcher, parliament)) ?? [];
	const delegateIds = [...new Set(questions.map((question) => question.delegate_id))];
	const delegates = await Promise.all(
		delegateIds.map(async (id) => errorToNull(await delegate_by_id(id, fetcher, parliament)))
	);
	const delegateById = new Map<number, QuestionDelegate>();
	for (const delegate of delegates) {
		const mapped = toQuestionDelegate(delegate);
		if (mapped) delegateById.set(mapped.id, mapped);
	}

	return questions.map((question) => ({
		question,
		delegate: delegateById.get(question.delegate_id) ?? null
	}));
}

export async function loadQuestionEntry(
	fetcher: typeof fetch,
	parliament: Parliament,
	slug: string
): Promise<DelegateQuestionView | null> {
	const parsed = parseQuestionSlug(slug);
	if (parsed === null) return null;

	if (!API_LIVE) {
		return (
			mockQuestionEntries(parliament).find((entry) => questionSlug(entry.question) === slug) ?? null
		);
	}

	const questions =
		errorToNull(await delegate_questions(parsed.delegateId, fetcher, parliament)) ?? [];
	const question =
		questions.find((entry) => new Date(entry.created_at).getTime() === parsed.createdAt) ?? null;
	if (question === null) return null;

	const delegate = errorToNull(await delegate_by_id(parsed.delegateId, fetcher, parliament));
	return { question, delegate: toQuestionDelegate(delegate) };
}
