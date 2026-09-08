import {
	delegate_by_id,
	delegate_question_by_id,
	delegate_question_recipient,
	delegate_questions_by_query_search,
	errorToNull
} from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type {
	Delegate,
	DelegateQuestionRecipient,
	PublicDelegateQuestion,
	DelegateQuestionsWithMaxPage
} from '$lib/types';
import type { DelegateQuestionView, QuestionDelegate } from './types';

function toQuestionDelegate(delegate: Delegate | null): QuestionDelegate | null {
	if (delegate === null) return null;
	return {
		id: delegate.id,
		name: delegate.name,
		party: delegate.current_party ?? delegate.party ?? null,
		image_url: delegate.image_url
	};
}

export interface QuestionSearchResult {
	entries: DelegateQuestionView[];
	entryCount: number;
	maxPage: number;
	updatedAt: string | null;
}

async function withDelegates(
	fetcher: typeof fetch,
	parliament: Parliament,
	questions: PublicDelegateQuestion[]
): Promise<DelegateQuestionView[]> {
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

export async function searchQuestionEntries(
	fetcher: typeof fetch,
	parliament: Parliament,
	query: string
): Promise<QuestionSearchResult> {
	const result: DelegateQuestionsWithMaxPage | null = errorToNull(
		await delegate_questions_by_query_search(query, fetcher, parliament)
	);
	if (result === null) {
		return { entries: [], entryCount: 0, maxPage: 1, updatedAt: null };
	}

	return {
		entries: await withDelegates(fetcher, parliament, result.delegate_questions),
		entryCount: result.entry_count,
		maxPage: result.max_page,
		updatedAt: result.updated_at
	};
}

export async function loadQuestionEntry(
	fetcher: typeof fetch,
	parliament: Parliament,
	questionId: string
): Promise<DelegateQuestionView | null> {
	const question = errorToNull(await delegate_question_by_id(questionId, fetcher, parliament));
	if (question === null) return null;

	const delegate = errorToNull(await delegate_by_id(question.delegate_id, fetcher, parliament));
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
