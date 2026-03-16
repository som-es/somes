import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { isHasError } from "$lib/api/api";
import { renew_token } from "$lib/api/authed";
import { jwtStore, loginDrawerOpenStore } from "$lib/caching/stores/stores.svelte";

export async function accountOrLogin() {
    const jwt = jwtStore.value;
    if (jwt) {
        if (isHasError(await renew_token())) {
            loginDrawerOpenStore.value = true;
        } else {
            goto(resolve('/user'));
        }
    } else {
        loginDrawerOpenStore.value = true;
    }
};