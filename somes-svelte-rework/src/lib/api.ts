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
	LoginResponseError
} from './types';

// const address = 'https://somes.at';
const address = 'http://127.0.0.1:3000';
// const address = "http://192.168.1.114:3000"

export function isHasError<T>(value: T | HasError): value is HasError {
    return (value as HasError).error !== undefined;
}

export function isLoginResponseError<T>(value: T | LoginResponseError): value is LoginResponseError {
    return (value as LoginResponseError).missing_email !== undefined;
}

export async function fetchSavely<T>(fn: () => Promise<Response>): Promise<T | HasError> {
	let response;
	try {
		response = await fn();
		const json = await response.json();
		if ('error' in json) {
			// return ;
		}
		return json;
	} catch (error) {
		console.log(response);
		console.log(`error: ${error}`);
		return { error: 'Error fetching data' };
	}
}

export async function seats(): Promise<Map<string, number[]> | HasError> {
    const response = await fetchSavely<{ [key: string]: number[] }>(() =>
        fetch(`${address}/seats`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            }
        })
    );

	if ("error" in response) {
		return response as HasError;
	}

    if (response) {
        return new Map<string, number[]>(Object.entries(response));
    }

	return { error: 'Error fetching data' };

}

export async function parties(): Promise<Party[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/parties`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function delegates(): Promise<Delegate[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/delegates`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function latest_vote_results(): Promise<VoteResult[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/latest_vote_results`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function all_gps(): Promise<LegisPeriod[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/all_gps`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function vote_results_per_page(
	page: number,
	filter: VoteResultFilter | null
): Promise<VoteResultsWithMaxPage | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/vote_results_per_page?page=${page}`, {
			method: 'POST', // only post because js fetch..
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(filter)
		})
	);
}

export async function delegate_interests(delegate_id: number): Promise<InterestShare[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/delegate_interests?delegate_id=${delegate_id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function vote_result_by_id(vote_result_id: string): Promise<VoteResult | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/vote_result_by_id?id=${vote_result_id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function delegates_at(date_at: Date): Promise<Delegate[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/delegates_at?at=${date_at}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function delegates_with_seats_near_date(date_at: Date, gp: string): Promise<Delegate[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/delegates_with_seats_near_date?at=${date_at}&period=${gp}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function vote_results_by_search(
	page: number,
	search: string
): Promise<VoteResultsWithMaxPage | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/vote_result_by_search?page=${page}&search=${search}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function walo_questions(): Promise<WaloQuestion[] | HasError> {
	return fetchSavely(() =>
		fetch(`${address}/walo_questions`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function login(
	email: string,
	password: string | null,
	hash_email: boolean | null,
): Promise<JWTInfo | HasError | LoginResponseError> {
	return fetchSavely(() =>
		fetch(`${address}/login`, {
			method: 'POST', // only post because js fetch..
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email: email,
				password: password,
				hash_email: hash_email,
			})
		})
	);
}
