import { get } from "svelte/store";
import { jwtStore } from "../stores/stores";
import type {
	CallToOrdersPerPartyDelegate,
	Delegate,
	DelegateByCallToOrders,
	InterestShare,
	Party,
	SpeakerByHours,
	VoteResult,
} from "./types";

export const address = "https://somes.at";
// export const address = "127.0.0.1";
// const address = "172.20.10.2";

export async function delegates(): Promise<Delegate[]> {
	let response = await fetch(`${address}/delegates_with_seats_near_date?at=2024-09-30&period=XXVII`, {
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

export async function speakers_by_hours(): Promise<SpeakerByHours[]> {
	let response = await fetch(`${address}/speakers_by_hours`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function delegates_by_call_to_orders(): Promise<DelegateByCallToOrders[]> {
	let response = await fetch(`${address}/delegates_by_call_to_orders`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function call_to_orders_per_party_delegates(): Promise<
	CallToOrdersPerPartyDelegate[]
> {
	let response = await fetch(`${address}/call_to_orders_per_party_delegates`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function parties(): Promise<Party[]> {
	let response = await fetch(`${address}/parties`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});

	return await response.json();
}

export async function delegates_by_call_to_orders_and_legis_period(
	period: string,
): Promise<DelegateByCallToOrders[]> {
	let response = await fetch(
		`${address}/delegates_by_call_to_orders_and_legis_period`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ period: period }),
		},
	);

	return await response.json();
}

export async function speakers_by_hours_and_legis_period(
	period: string,
): Promise<SpeakerByHours[]> {
	let response = await fetch(`${address}/speakers_by_hours_and_legis_period`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ period: period }),
	});

	return await response.json();
}

export async function delegate_interests(
	delegate_id: number
): Promise<InterestShare[]> {
		let response = await fetch(`${address}/delegate_interests?delegate_id=${delegate_id}`, {
		method: "GET",
		// headers: {
		// 	"Content-Type": "application/json",
		// },
	});

	return await response.json();
}
