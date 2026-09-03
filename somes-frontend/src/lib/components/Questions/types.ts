import type { PublicDelegateQuestion, PublicDelegateQuestionAnswer } from '$lib/types';

export interface QuestionDelegate {
	id: number;
	name: string;
	party: string | null;
	image_url: string | null;
}

export interface DelegateQuestionView {
	question: PublicDelegateQuestion;
	delegate: QuestionDelegate | null;
}

export function questionSlug(question: PublicDelegateQuestion): string {
	return `${question.delegate_id}-${new Date(question.created_at).getTime()}`;
}

export function parseQuestionSlug(slug: string): { delegateId: number; createdAt: number } | null {
	const [delegateId, createdAt] = slug.split('-').map(Number);
	if (!Number.isFinite(delegateId) || !Number.isFinite(createdAt)) return null;
	return { delegateId, createdAt };
}

export function latestAnswer(
	question: PublicDelegateQuestion
): PublicDelegateQuestionAnswer | null {
	return question.answers.at(-1) ?? null;
}
