import type { CallToOrdersPerPartyDelegate, Delegate, DelegateByCallToOrders, Party, SpeakerByHours, VoteResult } from "./types";

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
    let response = await fetch(`http://${address}:3000/delegates_by_call_to_orders`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function call_to_orders_per_party_delegates(): Promise<CallToOrdersPerPartyDelegate[]> {
    let response = await fetch(`http://${address}:3000/call_to_orders_per_party_delegates`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function parties(): Promise<Party[]> {
    let response = await fetch(`http://${address}:3000/parties`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}

export async function delegates_by_call_to_orders_and_legis_period(period: string): Promise<DelegateByCallToOrders[]> {
    let response = await fetch(`http://${address}:3000/delegates_by_call_to_orders_and_legis_period`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ period: period })
    });

    return await response.json();
}

export async function speakers_by_hours_and_legis_period(period: string): Promise<SpeakerByHours[]> {
    let response = await fetch(`http://${address}:3000/speakers_by_hours_and_legis_period`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ period: period })
    });

    return await response.json();
}