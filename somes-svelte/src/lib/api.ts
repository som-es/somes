import type { Delegate, SpeakerByHours, VoteResult } from "./types";

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
