import { address } from "../api";

export async function register(username: string, email: string, password: string) {
    let response = await fetch(`http://${address}:3000/signup`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ username: username, email: email, password: password })
    });

    return await response.json();
}

export async function login(username_or_email: string, password: string) {
    let response = await fetch(`http://${address}:3000/login`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ username_or_email: username_or_email, password: password })
    });
    return await response.json();
}