<script lang="ts">
	import { register } from '$lib/api/register';
    import { LightSwitch, popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';


    /*$: if ($bearer != null) {
        window.location.replace("./");
    }*/

    let username = "";
    let email = "";
    let pwd = "";
    let pwdVerify = "";
    let nameTaken = false
    $: userError = username === "" || nameTaken;
    $: userErrorMessage = username === "" ? "Benutzername darf nicht leer sein!" : "Benutzername existiert schon!"
    $: emailErrorMessage = email === "" ? "E-Mail darf nicht leer sein!" : "E-Mail existiert schon!"
    $: pwdError = pwd !== pwdVerify || pwd === "" || pwdVerify === "";
    $: pwdErrorMessage = pwd !== pwdVerify ? "Passwörter stimmen nicht überein!" : pwd === "" || pwdVerify === "" ? "Passwort darf nicht leer sein" : "";

    let pwdPopupSettings: PopupSettings = {
    	event: 'hover',
    	target: 'pwdErrorPopup'
    };
    let userPopupSettings: PopupSettings = {
    	event: 'hover',
    	target: 'userErrorPopup'
    };
</script>

<div class="login_container flex flex-col">
    <h2 class="text-center">Registrierung</h2>
    <label for="username">Benutzername</label>
    <input id="username" placeholder="gertrud21" type="text" />

    <label for="email">E-Mail</label>
    <input id="email" placeholder="dergertrud@gmail.com" type="text" />

    <label for="password">Passwort</label>
    <input id="password" type="password" />

    <label for="passwordVerify">Passwort wiederholen</label>
    <input id="passwordVerify" type="password" />
    <input type="submit" value="Registrieren" />
</div>

<style>

input[type="text"],
input[type="password"] {
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