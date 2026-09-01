import type { Decree } from '$lib/components/Delegates/Decrees/types';
import type { Locale } from '$lib/i18n';
import type {
	Delegate,
	DelegateQuestionRecipient,
	PublicDelegateQuestion,
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
	SpeechesWithMaxPage,
	PoliticalPosition,
	GovProposalDelegate,
	GeneralDelegateInfo,
	GovPropFilter,
	GovProposalsWithMaxPage,
	GeneralGovOfficialInfo,
	DelegatesWithMaxPage,
	PartyStates,
	InterjectionsWithMaxPage,
	SessionActivityOverview,
	PlenarySession
} from '../types';
import { getParliament, type Parliament } from './parliament';

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
	return (value as HasError)?.error !== undefined;
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

export async function justPost<T>(
	route: string,
	body: any,
	parliament: Parliament = getParliament(),
	fetcher: typeof fetch = fetch
): Promise<T | HasError> {
	return fetchSavely(() =>
		fetcher(`${url}${parliament}/${route}`, {
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

export async function getWithRoute<T>(
	route: string,
	parliament: Parliament = getParliament(),
	fetcher: typeof fetch = fetch
): Promise<T | HasError> {
	return fetchSavely(() =>
		fetcher(`${url}${parliament}/${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Encoding': 'gzip'
			}
		})
	);
}

export async function seats(
	parliament: Parliament = getParliament(),
	fetcher: typeof fetch = fetch
): Promise<Map<string, number[]> | HasError> {
	const response = await getWithRoute<{ [key: string]: number[] }>('seats', parliament, fetcher);

	if ('error' in response) {
		return response as HasError;
	}

	if (response) {
		return new Map(Object.entries(response));
	}

	return { error: 'Error fetching data', error_type: 'FetchError', field: '', meta: null };
}

export async function parties(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Party[] | HasError> {
	return getWithRoute('parties', parliament, fetcher);
}

export async function parties_per_gp(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Record<string, Party[]> | HasError> {
	return getWithRoute('parties_per_gp', parliament, fetcher);
}

export async function coalition_parties_per_gp(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Record<string, PartyStates> | HasError> {
	return getWithRoute('coalition_parties_per_gp', parliament, fetcher);
}

export async function delegates(): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>('v1/delegates/all_active');
}

export async function delegate_question_recipient(
	delegateId: number,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<DelegateQuestionRecipient | HasError> {
	return getWithRoute(
		`v1/delegate_questions/delegate/${delegateId}/question_recipient`,
		parliament,
		fetcher
	);
}

export async function all_delegate_questions(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<PublicDelegateQuestion[] | HasError> {
	return getWithRoute<PublicDelegateQuestion[]>('v1/delegate_questions', parliament, fetcher);
}

export async function delegate_questions(
	delegateId: number,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<PublicDelegateQuestion[] | HasError> {
	return getWithRoute<PublicDelegateQuestion[]>(
		`v1/delegate_questions/delegate/${delegateId}`,
		parliament,
		fetcher
	);
}

export async function latest_vote_results(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<VoteResult[] | HasError> {
	return getWithRoute<VoteResult[]>('v1/vote_results/latest', parliament, fetcher);
}

export async function latest_session_activity_overview(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<SessionActivityOverview | null | HasError> {
	return justPost<SessionActivityOverview | null>(
		'v1/statistics/latest_session_activity_overview',
		{},
		parliament,
		fetcher
	);
}

export async function all_gps(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<LegisPeriod[] | HasError> {
	return getWithRoute<LegisPeriod[]>('all_gps', parliament, fetcher);
}

export async function delegate_by_id(
	delegate_id: number,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Delegate | HasError> {
	return getWithRoute<Delegate>(`v1/delegates/id/${delegate_id}`, parliament, fetcher);
}

export async function delegate_interests(delegate_id: number): Promise<InterestShare[] | HasError> {
	return getWithRoute<InterestShare[]>(`delegate_interests?delegate_id=${delegate_id}`);
}

export async function general_delegate_info(
	delegate_id: number,
	language: Locale = 'de'
): Promise<GeneralDelegateInfo | HasError> {
	return getWithRoute<GeneralDelegateInfo>(
		`v1/delegates/extend/${delegate_id}?language=${language}`
	);
}

export async function delegate_qa(delegate_id: number): Promise<DelegateQA[] | HasError> {
	return getWithRoute<DelegateQA[]>(`v1/delegates/delegate_qa/${delegate_id}`);
}

export async function vote_result_by_id(
	vote_result_id: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<VoteResult | HasError> {
	return getWithRoute<VoteResult>(`v1/vote_results/id/${vote_result_id}`, parliament, fetcher);
}

export async function vote_result_by_path(
	gp: string,
	ityp: string,
	inr: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<VoteResult | HasError> {
	return getWithRoute<VoteResult>(`v1/vote_results/${gp}/${ityp}/${inr}`, parliament, fetcher);
}

export async function delegates_at(
	date_at: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Delegate[] | HasError> {
	return getWithRoute(`v1/delegates/all_at_date?at=${date_at}`, parliament, fetcher);
}

export async function delegates_search_persons(
	page: number,
	entries_per_page: number,
	name: string | null = null,
	searchPeriods: string[] = [],
	searchParties: string[] = [],
	onlyGov: boolean | null = null,
	mindPreviousPartyMembership: boolean = true,
	hasActiveMandate: boolean | null = null,
	searchCountries: string[] = [],
	fetcher: typeof fetch = fetch
): Promise<DelegatesWithMaxPage | HasError> {
	let query = `v1/delegates/search?page=${page}&entries_per_page=${entries_per_page}`;
	if (name) query += `&search=${encodeURIComponent(name)}`;

	searchPeriods.forEach((searchPeriod, i) => {
		const gps = onlyGov ? (onlyGov ? 'active_gov_gps' : 'active_nr_gps') : 'active_gps';
		query += `&${gps}[in][${i}]=${encodeURIComponent(searchPeriod)}`;
	});

	searchParties.forEach((party, i) => {
		if (mindPreviousPartyMembership) {
			query += `&mandates[0][party][in][${i}]=${encodeURIComponent(party)}`;
		} else {
			query += `&party[in][${i}]=${encodeURIComponent(party)}`;
		}
	});

	searchCountries.forEach((country, i) => {
		query += `&constituency[in][${i}]=${encodeURIComponent(country)}`;
	});

	if (onlyGov !== null) {
		query += `&mandates[0][is_gov_official][eq]=${onlyGov}`;
	}

	if (hasActiveMandate !== null) {
		query += `&is_active[eq]=${hasActiveMandate}`;
	}

	return getWithRoute<DelegatesWithMaxPage>(query, getParliament(), fetcher);
}

export async function gov_officials_at(
	date_at: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Delegate[] | HasError> {
	return getWithRoute(`v1/delegates/gov_officials/all_at_date?at=${date_at}`, parliament, fetcher);
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
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<GovProposalDelegate[] | HasError> {
	return getWithRoute(`v1/gov_proposals/latest?days=${days}`, parliament, fetcher);
}
export async function latest_decrees(
	days: number,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Decree[] | HasError> {
	return getWithRoute(`v1/decrees/latest?days=${days}`, parliament, fetcher);
}

export async function speeches_by_delegate_per_page(
	delegate_id: number,
	page: number
): Promise<SpeechesWithMaxPage | HasError> {
	return getWithRoute<SpeechesWithMaxPage>(
		`v1/delegates/speeches_per_page?delegate_id=${delegate_id}&page=${page}`
	);
}

export async function interjections_made_by_delegate_per_page(
	delegate_id: number,
	page: number
): Promise<InterjectionsWithMaxPage | HasError> {
	return getWithRoute<InterjectionsWithMaxPage>(
		`v1/delegates/interjections/made?delegate_id=${delegate_id}&page=${page}`
	);
}

export async function interjections_received_by_delegate_per_page(
	delegate_id: number,
	page: number
): Promise<InterjectionsWithMaxPage | HasError> {
	return getWithRoute<InterjectionsWithMaxPage>(
		`v1/delegates/interjections/received?delegate_id=${delegate_id}&page=${page}`
	);
}

export async function delegates_with_seats_near_date(
	date_at: Date,
	gp: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Delegate[] | HasError> {
	return getWithRoute<Delegate[]>(
		`v1/delegates/all_at_date_with_seat_info?at=${date_at}&period=${gp}`,
		parliament,
		fetcher
	);
}

export async function get_eurovoc_topics(
	parliament: Parliament = getParliament(),
	fetcher: typeof fetch = fetch
): Promise<UniqueTopic[] | HasError> {
	return getWithRoute<UniqueTopic[]>('eurovoc_topics', parliament, fetcher);
}

export async function get_topics(): Promise<UniqueTopic[] | HasError> {
	return getWithRoute<UniqueTopic[]>('topics');
}

export async function walo_questions(): Promise<WaloQuestion[] | HasError> {
	return fetchSavely(() =>
		fetch(`${url}walo_questions`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Encoding': 'gzip'
			}
		})
	);
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

export async function departments_per_gp(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Record<string, string[]> | HasError> {
	return getWithRoute(`departments_per_gp`, parliament, fetcher);
}
export async function gov_proposals_by_search(
	query: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<GovProposalsWithMaxPage | HasError> {
	return getWithRoute(`v1/gov_proposals/search?${query}`, parliament, fetcher);
}

export async function vote_results_by_search(
	page: number,
	search: string,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return justPost(`v1/vote_results/search?page=${page}&search=${search}`, filter);
}

export async function vote_results_by_query_search(
	query: string,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<VoteResultsWithMaxPage | HasError> {
	return getWithRoute(`v1/vote_results/search?${query}`, parliament, fetcher);
}

export async function plenary_sessions_per_gp(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Record<string, PlenarySession[]> | HasError> {
	return getWithRoute(`plenary_sessions_per_gp`, parliament, fetcher);
}
