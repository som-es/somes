import type { Decree } from '$lib/components/Delegates/Decrees/types';
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
	PoliticalPosition,
	GovProposalDelegate,
	GeneralDelegateInfo,
	GovPropFilter,
	GovProposalsWithMaxPage,
	GeneralGovOfficialInfo
} from '../types';

// const address = 'https://somes.at';
// const address = 'http://127.0.0.1:3000';
export const address = import.meta.env.VITE_API_URL;

// const address = "http://192.168.1.114:3000"
//
export const url = `${address}/api/`;

export function toActualDateString(date: Date): string {
	return date.toISOString().split('T')[0];
}

export function isHasError<T>(value: T | HasError): value is HasError {
	return (value as HasError).error !== undefined;
}

export function isThere<T>(value: T | null): boolean {
	if (value) {
		return true;
	} else {
		return false;
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
		console.log(`error: ${error}, response: ${response?.url}`);
		return { error: 'Error data', error_type: 'FetchError', field: '', meta: null };
	}
}

export async function justPost<T>(route: string, body: any, country = 'at/'): Promise<T | HasError> {
	return fetchSavely(() =>
		fetch(`${url}${country}${route}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		})
	);
}
export async function justPostStatistics<T>(route: string, body: any): Promise<T | HasError> {
	return justPost(`v1/statistics/${route}`, body);
}

export async function getWithRoute<T>(route: string, country = 'at/', fetcher: typeof fetch = fetch): Promise<T | HasError> {
	return fetchSavely(() =>
		fetcher(`${url}${country}${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function seats(fetcher: typeof fetch = fetch): Promise<Map<string, number[]> | HasError> {
	const response = await getWithRoute<{ [key: string]: number[] }>('seats', 'at/', fetcher);

	if ('error' in response) {
		return response as HasError;
	}

	if (response) {
		return new Map(Object.entries(response));
	}

	return { error: 'Error fetching data', error_type: 'FetchError', field: '', meta: null };
}

export async function parties(): Promise<Party[] | HasError> {
	return getWithRoute('parties');
}

export async function delegates(): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>('v1/delegates/all_active');
}

export async function latest_vote_results(fetcher: typeof fetch = fetch): Promise<VoteResult[] | HasError> {
	return getWithRoute<VoteResult[]>('v1/vote_results/latest', "at/", fetcher);
}

export async function all_gps(): Promise<LegisPeriod[] | HasError> {
	return getWithRoute<LegisPeriod[]>('all_gps');
}

export async function delegate_by_id(delegate_id: number, fetcher: typeof fetch = fetch): Promise<Delegate | HasError> {
	return getWithRoute<Delegate>(`v1/delegates/id/${delegate_id}`, 'at/', fetcher);
}

export async function delegate_interests(delegate_id: number): Promise<InterestShare[] | HasError> {
	return getWithRoute<InterestShare[]>(`delegate_interests?delegate_id=${delegate_id}`);
}

export async function general_delegate_info(
	delegate_id: number
): Promise<GeneralDelegateInfo | HasError> {
	return getWithRoute<GeneralDelegateInfo>(`v1/delegates/extend/${delegate_id}`);
}

export async function delegate_qa(delegate_id: number): Promise<DelegateQA[] | HasError> {
	return getWithRoute<DelegateQA[]>(`v1/delegates/delegate_qa/${delegate_id}`);
}

export async function vote_result_by_id(vote_result_id: string): Promise<VoteResult | HasError> {
	return getWithRoute<VoteResult>(`v1/vote_results/id/${vote_result_id}`);
}

export async function vote_result_by_path(
	gp: string,
	ityp: string,
	inr: string,
	fetcher: typeof fetch = fetch
): Promise<VoteResult | HasError> {
	return getWithRoute<VoteResult>(`v1/vote_results/${gp}/${ityp}/${inr}`, "at/", fetcher);
}

export async function delegates_at(
	date_at: string,
	fetcher: typeof fetch = fetch
): Promise<Delegate[] | HasError> {
	return getWithRoute(`v1/delegates/all_at_date?at=${date_at}`, "at/", fetcher);
}

export async function gov_officials_at(
	date_at: string,
	fetcher: typeof fetch = fetch
): Promise<Delegate[] | HasError> {
	return getWithRoute(`v1/delegates/gov_officials/all_at_date?at=${date_at}`, "at/", fetcher);
}

export async function gov_proposals_by_official(
	delegate_id: number
): Promise<GovProposal[] | HasError> {
	return getWithRoute(`v1/delegates/gov_officials/gov_proposals/${delegate_id}`);
}

export async function general_gov_official_info(
	delegate_id: number
): Promise<GeneralGovOfficialInfo | HasError> {
	return getWithRoute(`v1/delegates/gov_officials/extend/${delegate_id}`);
}

export async function latest_ministrial_proposals(
	days: number,
	fetcher: typeof fetch = fetch
): Promise<GovProposalDelegate[] | HasError> {
	return getWithRoute(`v1/gov_proposals/latest?days=${days}`, "at/", fetcher);
}
export async function latest_decrees(
	days: number,
	fetcher: typeof fetch = fetch
): Promise<Decree[] | HasError> {
	return getWithRoute(`v1/decrees/latest?days=${days}`, "at/", fetcher);
}

export async function speeches_by_delegate_per_page(
	delegate_id: number,
	page: number
): Promise<SpeechesWithMaxPage | HasError> {
	return getWithRoute<SpeechesWithMaxPage>(
		`v1/delegates/speeches_per_page?delegate_id=${delegate_id}&page=${page}`
	);
}

export async function delegates_with_seats_near_date(
	date_at: Date,
	gp: string,
	fetcher: typeof fetch = fetch
): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>(
		`v1/delegates/all_at_date_with_seat_info?at=${date_at}&period=${gp}`, "at/", fetcher
	);
}

export async function get_eurovoc_topics(): Promise<UniqueTopic[] | HasError> {
	return getWithRoute<UniqueTopic[]>('eurovoc_topics');
}

export async function get_topics(): Promise<UniqueTopic[] | HasError> {
	return getWithRoute<UniqueTopic[]>('topics');
}

export async function walo_questions(): Promise<WaloQuestion[] | HasError> {
	return getWithRoute<WaloQuestion[]>('walo_questions', "");
}

export async function vote_results_per_page(
	page: number,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return justPost(`v1/vote_results/live/?page=${page}`, filter);
}

export async function gov_proposals_per_page(
	page: number,
	filter: GovPropFilter | null
): Promise<GovProposalsWithMaxPage | HasError> {
	return justPost(`v1/gov_proposals/live/?page=${page}`, filter);
}

export async function gov_proposals_by_search(
	page: number,
	search: string,
	filter: GovPropFilter | null
): Promise<GovProposalsWithMaxPage | HasError> {
	return justPost(`v1/gov_proposals/search?page=${page}&search=${search}`, filter);
}

export async function vote_results_by_search(
	page: number,
	search: string,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return justPost(`v1/vote_results/search?page=${page}&search=${search}`, filter);
}
