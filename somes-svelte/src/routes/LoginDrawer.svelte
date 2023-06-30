<script lang="ts">
	import { login } from '$lib/api/register';

    // import { login } from '$lib/api';
	import { drawerStore } from '@skeletonlabs/skeleton';
	// import Login from 'svelte-google-materialdesign-icons/Login.svelte';

    let username_or_email = "";
    let pwd = "";


</script>

<div class="login_container flex flex-col">
    <h2 class="text-center">Anmeldung</h2>
    <label for="username">Benutzername oder E-Mail</label>
    <input id="username" placeholder="'gertrud21' oder 'dergertrud@gmail.com'" type="text" bind:value={username_or_email} />

    <label for="password">Passwort</label>
    <input id="password" type="password" bind:value={pwd} />

    <input type="submit" value="Anmelden" on:click={async () => {
        let res = await login(username_or_email, pwd);
        console.log(res);
        if (res) {
            window.location.href = "./";
        }
    }} />

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
</style>
