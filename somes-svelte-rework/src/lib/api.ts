import type {
	Delegate,
	HasError,
	InterestShare,
	VoteResultFilter,
	LegisPeriod,
	Party,
	VoteResult,
	VoteResultsWithMaxPage
} from './types';

// const address = "http://somes.at:3000"
const address = 'http://127.0.0.1:3000';
// const address = "http://192.168.1.114:3000"

export async function fetchSavely<T>(fn: () => Promise<Response>): Promise<T | null> {
	try {
		let response = await fn();
		const json = await response.json();
		if ('error' in json) {
			return null;
		}
		return json;
	} catch (error) {
		console.log(error);
		return null;
	}
}

export async function parties(): Promise<Party[] | null> {
	return fetchSavely(() =>
		fetch(`${address}/parties`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function delegates(): Promise<Delegate[] | null> {
	return fetchSavely(() =>
		fetch(`${address}/delegates`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function latest_vote_results(): Promise<VoteResult[] | null> {
	return fetchSavely(() =>
		fetch(`${address}/latest_vote_results`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function all_gps(): Promise<LegisPeriod[] | null> {
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
): Promise<VoteResultsWithMaxPage | null> {
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

export async function delegate_interests(delegate_id: number): Promise<InterestShare[] | null> {
	return fetchSavely(() =>
		fetch(`${address}/delegate_interests?delegate_id=${delegate_id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function vote_result_by_id(vote_result_id: string): Promise<VoteResult | null> {
	return fetchSavely(() =>
		fetch(`${address}/vote_result_by_id?id=${vote_result_id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}

export async function delegates_at(date_at: Date): Promise<Delegate[] | null> {
	return fetchSavely(() =>
		fetch(`${address}/delegates_at?at=${date_at}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
	);
}
