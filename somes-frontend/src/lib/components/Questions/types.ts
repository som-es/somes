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

export function latestAnswer(
	question: PublicDelegateQuestion
): PublicDelegateQuestionAnswer | null {
	return question.answers.at(-1) ?? null;
}
