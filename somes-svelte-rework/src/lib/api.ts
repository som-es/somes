import type {
	Delegate,
	HasError,
	InterestShare,
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
		return await response.json();
	} catch (error) {
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

export async function vote_results_per_page(page: number): Promise<VoteResultsWithMaxPage | null> {
	return fetchSavely(() =>
		fetch(`${address}/vote_results_per_page?page=${page}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
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
