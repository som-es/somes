
import { get } from 'svelte/store';

import type {
	Delegate,
	HasError,
	InterestShare,
	VoteResultFilter,
	LegisPeriod,
	Party,
	VoteResult,
	VoteResultsWithMaxPage,
	WaloQuestion,
	JWTInfo,
	LoginResponseError,
	DelegateQA,
	Topic,
	UniqueTopic,
	GovProposal,
	Speech,
	SpeechesWithMaxPage,
	PoliticalPosition
} from '../types';
import { jwtStore } from '../caching/stores/stores';

// const address = 'https://somes.at';
// const address = 'http://127.0.0.1:3000';
export const address = import.meta.env.VITE_API_URL;

// const address = "http://192.168.1.114:3000"

export function isHasError<T>(value: T | HasError): value is HasError {
	return (value as HasError).error !== undefined;
}

export function isThere<T>(value: T | null): boolean {
	if (value) {
		return true
	} else {
		return false
	}
}

export function isLoginResponseError<T>(
	value: T | LoginResponseError
): value is LoginResponseError {
	return (value as LoginResponseError).missing_email !== undefined;
}

export function errorToNull<T>(input: T | HasError): T | null {
	if (isHasError(input)) {
		return null;
	} else {
		return input;
	}
}

export async function fetchSavely<T>(fn: () => Promise<Response>): Promise<T | HasError> {
	let response;
	try {
		response = await fn();
		const json = await response.json();
		// if ('error' in json) {
		// return ;
		// }
		return json;
	} catch (error) {
		console.log(response);
		console.log(`error: ${error}`);
		return { error: 'Error fetching data' };
	}
}

export async function justPost<T>(route: string, body: any): Promise<T | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/${route}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(body)
		})
	);
}

export async function getWithRoute<T>(route: string): Promise<T | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function seats(): Promise<Map<string, number[]> | HasError> {
	const response = await getWithRoute<{ [key: string]: number[] }>('seats');

	if ('error' in response) {
		return response as HasError;
	}

	if (response) {
		return new Map(Object.entries(response));
	}

	return { error: 'Error fetching data' };
}

export async function parties(): Promise<Party[] | HasError> {
	return getWithRoute('parties');
}

export async function delegates(): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>('delegates');
}

export async function latest_vote_results(): Promise<VoteResult[] | HasError> {
	return getWithRoute<VoteResult[]>('latest_vote_results');
}

export async function all_gps(): Promise<LegisPeriod[] | HasError> {
	return getWithRoute<LegisPeriod[]>('all_gps');
}

export async function delegate_interests(delegate_id: number): Promise<InterestShare[] | HasError> {
	return getWithRoute<InterestShare[]>(`delegate_interests?delegate_id=${delegate_id}`);
}

export async function delegate_qa(delegate_id: number): Promise<DelegateQA[] | HasError> {
	return getWithRoute<DelegateQA[]>(`delegate_qa?delegate_id=${delegate_id}`);
}

export async function vote_result_by_id(vote_result_id: string): Promise<VoteResult | HasError> {
	return getWithRoute<VoteResult>(`vote_result_by_id?id=${vote_result_id}`);
}

export async function delegates_at(date_at: Date): Promise<Delegate[] | HasError> {
	return getWithRoute(`delegates_at?at=${date_at}`);
}

export async function gov_officials_at(date_at: Date): Promise<Delegate[] | HasError> {
	return getWithRoute(`gov_officials_at?at=${date_at}`);
}

export async function gov_proposals_by_official(delegate_id: number): Promise<GovProposal[] | HasError> {
	return getWithRoute(`gov_proposals_by_official?delegate_id=${delegate_id}`);
}

export async function delegate_political_position(delegate_id: number): Promise<PoliticalPosition | HasError> {
	return getWithRoute(`delegate_political_position?delegate_id=${delegate_id}`);
}

export async function delegate_political_questions(delegate_id: number): Promise<DelegateQA[] | HasError> {
	return getWithRoute(`delegate_political_questions?delegate_id=${delegate_id}`);
}

export async function speeches_by_delegate_per_page(
	delegate_id: number,
	page: number
): Promise<SpeechesWithMaxPage | HasError> {
	return getWithRoute<SpeechesWithMaxPage>(
		`speeches_by_delegate_per_page?delegate_id=${delegate_id}&page=${page}`
	);
}

export async function delegates_with_seats_near_date(
	date_at: Date,
	gp: string
): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>(
		`delegates_with_seats_near_date?at=${date_at}&period=${gp}`
	);
}

export async function get_topics(): Promise<UniqueTopic[] | HasError> {
	return getWithRoute<UniqueTopic[]>('topics');
}

export async function walo_questions(): Promise<WaloQuestion[] | HasError> {
	return getWithRoute<WaloQuestion[]>('walo_questions');
}

export async function vote_results_per_page(
	page: number,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/vote_results_per_page?page=${page}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(filter)
		})
	);
}

export async function vote_results_by_search(
	page: number,
	search: string,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/vote_result_by_search?page=${page}&search=${search}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(filter)
		})
	);
}
