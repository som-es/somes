import type { Parliament } from '$lib/api/parliament';

// TODO: replace with the real API
export interface PoliticianQuestionAnswer {
	delegateName: string;
	party: string;
	text: string;
	date: string; // YYYY-MM-DD
}

export interface PoliticianQuestion {
	id: number;
	parliament: Parliament;
	askedBy: string;
	date: string; // YYYY-MM-DD
	question: string; // headline
	text: string; // full message from the asker
	topics: string[];
	answer: PoliticianQuestionAnswer | null;
}
