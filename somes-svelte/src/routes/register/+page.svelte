<script lang="ts">
	import { register } from '$lib/api/register';
	import { drawerStore, type PopupSettings } from '@skeletonlabs/skeleton';
	import { verificationMailStore } from '../../stores/stores';
	import { loginDrawerSettings } from '$lib/constants';

    /*$: if ($bearer != null) {
        window.location.replace("./");
    }*/

    let username = "";
    let email = "";
    let pwd = "";
    let pwdVerify = "";
    $: userErrorMessage = username === "" ? "Benutzername darf nicht leer sein!" : "";
    $: emailErrorMessage = email === "" ? "E-Mail darf nicht leer sein!" : "";
    $: pwdErrorMessage = pwd !== pwdVerify ? "Passwörter stimmen nicht überein!" : pwd === "" || pwdVerify === "" ? "Passwort darf nicht leer sein!" : "";

    let pwdPopupSettings: PopupSettings = {
    	event: 'hover',
    	target: 'pwdErrorPopup'
    };
    let userPopupSettings: PopupSettings = {
    	event: 'hover',
    	target: 'userErrorPopup'
    };

    let waitForVerification = false;
</script>

{#if !waitForVerification}
<div class="login_container flex flex-col text-black">
    <h2 class="text-center">Registrierung</h2>
    <label for="username">Benutzername</label>
    <span class="text-red-500">{userErrorMessage}</span>
    <input id="username" class="down" placeholder="gertrud21" type="text" bind:value={username} />

    <label for="email">E-Mail</label>
    <span class="text-red-500">{emailErrorMessage}</span>
    <input id="email" class="down" placeholder="dergertrud@gmail.com" type="text" bind:value={email} />

    <label for="password">Passwort</label>
    <input id="password" class="down" type="password" bind:value={pwd} />

    <label for="passwordVerify">Passwort wiederholen</label>
    <input id="passwordVerify" type="password" bind:value={pwdVerify} />
    <span class="down text-red-500">{pwdErrorMessage}</span>

    <input type="button" value="Registrieren" on:click={async () => {
        if (email === "" || pwd === "" || username === "") {
            return;
        }

        let res = await register(username, email, pwd);
        if (res == null) {
            waitForVerification = true;
            verificationMailStore.set(email);
            return;
        }
        if (res.username_taken) {
            userErrorMessage = "Benutzername bereits vergeben!";
        }
        if (res.email_taken) {
            emailErrorMessage = "E-Mail bereits vergeben!";
        }

        if (res.invalid_email) {
            emailErrorMessage = "Ungültige E-Mail!";
        }

        if (res.insufficient_password) {
            pwdErrorMessage = "Passwort muss bestimmte Kriterien erfüllen! (definiere, grafisch darstellen..)";
        }
    }} />
</div>
{:else}

<div class="login_container flex flex-col ">
    <h2 class="text-center">Registrierung verifizieren</h2>
    <h4 class="text-center mt-2">Verifizierungsmail an <span class="font-bold text-green-400">{email}</span> versendet.</h4>
    <p class="mt-1 text-center">Bitte überprüfe deine E-Mail und klicke auf den Link, um deine Registrierung abzuschließen</p>
</div>
<div class="login_container">
    <p class="text-center"> Nach dem Klicken des Links kann sich hier angemeldet werden:</p>
    <input on:click={_ => drawerStore.open(loginDrawerSettings)} class="w-full mt-1" type="button" value="Anmelden" />
</div>
{/if}

<style>

.down {
    margin-bottom: 20px;
}

input[type="text"],
input[type="password"] {
	padding: 10px;
	border: none;
	border-radius: 5px;
	box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
	font-size: 16px;
}

.login_container {
	background-color: #fff;
	margin: 20px auto;
    width: 600px;
	max-width: 95%;
	padding: 20px;
	border-radius: 10px;
	box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.2);
}


input[type="button"] {
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

input[type="button"]:hover {
	background-color: #fff;
	color: rgb(var(--color-tertiary-500));
}
</style>