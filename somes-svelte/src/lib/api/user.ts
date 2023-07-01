import { address } from "$lib/api";
import type { HasError, UserInfo } from "$lib/types";
import { get } from "svelte/store";
import { jwtStore } from "../../stores/stores";

export async function getUserByJWT(): Promise<UserInfo | HasError> {
    let response = await fetch(`http://${address}:3000/user`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${get(jwtStore)}` 
        },
    });

    return await response.json();
}

export async function maybeGetUser(): Promise<(UserInfo | HasError) | null> {
    const jwt = get(jwtStore);
    if (jwt == null) {
        return null;
    }
    return await getUserByJWT();
}