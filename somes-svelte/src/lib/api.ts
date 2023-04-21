import type { Delegate } from "./types";

export async function delegates(): Promise<Delegate[]> {
    let response = await fetch("http://127.0.0.1:3000/delegates", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });

    return await response.json();

    /*return json.map((delegate: any) => {
        return {
            id: delegate.id,
            name: delegate.name,
            party: delegate.party,
            constituency: delegate.constituency,
            council: delegate.council,
            seat_row: delegate.seat_row,
            seat_col: delegate.seat_col,
        }
    });*/
}