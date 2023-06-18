import type { Delegate, DelegateByCallToOrders, SpeakerByHours, VoteResult } from "./types";

const address = "127.0.0.1";
// const address = "172.20.10.2";

export async function delegates(): Promise<Delegate[]> {
    let response = await fetch(`http://${address}:3000/delegates`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });
    return await response.json();
}

export async function latest_vote_results(): Promise<VoteResult[]> {
    let response = await fetch(`http://${address}:3000/latest_vote_results`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function speakers_by_hours(): Promise<SpeakerByHours[]> {
    let response = await fetch(`http://${address}:3000/speakers_by_hours`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function delegates_by_call_to_orders(): Promise<DelegateByCallToOrders[]> {
    let response = await fetch(`http://${address}:3000/speakers_by_hours`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function call_to_orders_per_party_delegates(): Promise<DelegateByCallToOrders[]> {
    let response = await fetch(`http://${address}:3000/call_to_orders_per_party_delegates`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}
