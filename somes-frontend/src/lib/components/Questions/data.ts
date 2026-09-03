import {
	all_delegate_questions,
	delegate_by_id,
	delegate_question_recipient,
	delegate_questions,
	errorToNull
} from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type { Delegate, DelegateQuestionRecipient } from '$lib/types';
import { parseQuestionSlug, type DelegateQuestionView, type QuestionDelegate } from './types';

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

	const questions =
		errorToNull(await delegate_questions(parsed.delegateId, fetcher, parliament)) ?? [];
	const question =
		questions.find((entry) => new Date(entry.created_at).getTime() === parsed.createdAt) ?? null;
	if (question === null) return null;

	const delegate = errorToNull(await delegate_by_id(parsed.delegateId, fetcher, parliament));
	return { question, delegate: toQuestionDelegate(delegate) };
}

export async function loadQuestionDelegate(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegateId: number
): Promise<QuestionDelegate | null> {
	return toQuestionDelegate(errorToNull(await delegate_by_id(delegateId, fetcher, parliament)));
}

// Full delegate for rendering a real DelegateCard next to a question.
export async function loadFullQuestionDelegate(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegateId: number
): Promise<Delegate | null> {
	return errorToNull(await delegate_by_id(delegateId, fetcher, parliament));
}

export async function loadQuestionRecipient(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegate: QuestionDelegate
): Promise<DelegateQuestionRecipient | null> {
	return errorToNull(await delegate_question_recipient(delegate.id, fetcher, parliament));
}
