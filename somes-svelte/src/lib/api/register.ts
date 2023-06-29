

import { address } from "../api";


export async function register(username: string, email: string, password: string) {
    let response = await fetch(`http://${address}:3000/register`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ username, email, password })
    });

    return await response.json();
}
