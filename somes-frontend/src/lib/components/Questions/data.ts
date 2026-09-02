import {
	all_delegate_questions,
	delegate_by_id,
	delegate_question_recipient,
	delegate_questions,
	errorToNull
} from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type { Delegate, DelegateQuestionRecipient } from '$lib/types';
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
export const QUESTIONS_API_LIVE: boolean = true;
const API_LIVE = QUESTIONS_API_LIVE;

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

export async function loadQuestionDelegate(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegateId: number
): Promise<QuestionDelegate | null> {
	// Mock ids resolve from the mock entries; everything else (e.g. real ids
	// linked from a DelegateCard) falls through to the live delegates API.
	if (!API_LIVE) {
		const mocked = mockQuestionEntries(parliament).find(
			(entry) => entry.delegate?.id === delegateId
		)?.delegate;
		if (mocked) return mocked;
	}

	return toQuestionDelegate(errorToNull(await delegate_by_id(delegateId, fetcher, parliament)));
}

// Full delegate for rendering a real DelegateCard next to a question. Mock-only
// ids have no API record, so they get a stub built from the mock delegate.
export async function loadFullQuestionDelegate(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegateId: number
): Promise<Delegate | null> {
	if (!API_LIVE) {
		const mocked = mockQuestionEntries(parliament).find(
			(entry) => entry.delegate?.id === delegateId
		)?.delegate;
		if (mocked) return mockFullDelegate(mocked);
	}

	return errorToNull(await delegate_by_id(delegateId, fetcher, parliament));
}

function mockFullDelegate(delegate: QuestionDelegate): Delegate {
	return {
		id: delegate.id,
		name: delegate.name,
		party: delegate.party ?? 'OK',
		current_party: delegate.party ?? 'OK',
		image_url: null,
		image_copyright: null,
		constituency: '',
		council: '',
		seat_row: null,
		seat_col: null,
		gender: null,
		is_active: null,
		birthdate: null,
		active_since: new Date(),
		divisions: null,
		mandates_at_time: null,
		active_mandates: null,
		mandates: null,
		active_gps: null,
		active_nr_gps: null,
		active_gov_gps: null
	};
}

export async function loadQuestionRecipient(
	fetcher: typeof fetch,
	parliament: Parliament,
	delegate: QuestionDelegate
): Promise<DelegateQuestionRecipient | null> {
	if (!API_LIVE) {
		return { delivery: 'delegate', recipient_name: delegate.name };
	}

	return errorToNull(await delegate_question_recipient(delegate.id, fetcher, parliament));
}
