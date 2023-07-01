<script lang="ts">
	import { login } from '$lib/api/register';
    import type { JWT, HasError } from '$lib/types';

    // import { login } from '$lib/api';
	import { drawerStore } from '@skeletonlabs/skeleton';
	import { jwtStore } from '../stores/stores';
	import { redirect } from '@sveltejs/kit';
	import { goto } from '$app/navigation';
	// import Login from 'svelte-google-materialdesign-icons/Login.svelte';

    let username_or_email = "";
    let pwd = "";

    let invalidCreds = "";

    const onLogin = async () => {
        let res: JWT | HasError = await login(username_or_email, pwd);
        // check if res is a JWT or HasError
        if ("error" in res) {
            invalidCreds = "Benutzername oder Passwort falsch!"
        } else {
            invalidCreds = ""
            jwtStore.set(res.access_token);
            drawerStore.close();
            goto("/home");
        }
    }


</script>

<div class="login_container flex flex-col">
    <span on:click={
        () => {
            drawerStore.close();
        }
    } style="font-size: 27px" class="w-5 font-bold unselectable">&#x2715</span><h2 class="text-center mb-4">Anmeldung</h2>
    <label for="username">Benutzername oder E-Mail</label>
    <input id="username" placeholder="'gertrud21' oder 'dergertrud@gmail.com'" type="text" bind:value={username_or_email} />

    <label for="password">Passwort</label>
    <input id="password" placeholder="password" type="password" bind:value={pwd} />

    <input type="submit" value="Anmelden" on:click={onLogin} />
    <span class="text-red-500">{invalidCreds}</span>    

</div>


<style>

input[type="text"],
input[type="password"] {
    flex-grow: 1;
    padding: 10px;
    margin-bottom: 20px;
    border: none;
    border-radius: 5px;
    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
    font-size: 16px;
}

.login_container {
    background-color: #fff;
    margin: 20px auto;
    max-width: 500px;
    padding: 20px;
    border-radius: 10px;
    box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.2);
}

input[type="submit"] {
    /* background-color: #5c5cd6; */
    background-color: rgb(var(--color-tertiary-500));
    color: #fff;
    padding: 10px;
    border: none;
    border-radius: 5px;
    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
    font-size: 20px;
    cursor: pointer;
    transition: all 0.3s ease;
}

input[type="submit"]:hover {
    background-color: #fff;
    color: rgb(var(--color-tertiary-500));
}

.unselectable {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    cursor: pointer;
}
</style>
