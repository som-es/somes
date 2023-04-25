import type { Delegate, VoteResult } from "./types";

const address = "127.0.0.1";

export async function delegates(): Promise<Delegate[]> {
    let response = await fetch(`http://${address}:3000/delegates`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });
    return await response.json();
}

export async function latest_legis_inits_and_votes(): Promise<VoteResult[]> {
    let response = await fetch(`http://${address}:3000/latest_legis_inits_and_votes`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();
}
