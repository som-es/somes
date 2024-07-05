import type { Delegate, Party, VoteResult } from "./types";

// const address = "http://somes.at:3000"
const address = "http://127.0.0.1:3000"

export async function parties(): Promise<Party[]> {
	let response = await fetch(`${address}/parties`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function delegates(): Promise<Delegate[]> {
	let response = await fetch(`${address}/delegates`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});
	return await response.json();
}

export async function latest_vote_results(): Promise<VoteResult[]> {
	let response = await fetch(`${address}/latest_vote_results`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function vote_results_per_page(page: number): Promise<VoteResult[]> {
	let response = await fetch(`${address}/vote_results_per_page?page=${page}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

