<script lang="ts">
	import type { JWTInfo, HasError, LoginResponseError } from '$lib/types';
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	// import { login } from '$lib/api';
	import { goto } from '$app/navigation';
	import { get } from 'svelte/store';
	import { browser } from '$app/environment';
	import { base } from '$app/paths';
	import { gotoHistory } from '$lib/goto';
	import { isHasError, isLoginResponseError, login } from '$lib/api';
	import SButton from '../UI/SButton.svelte';
	import { jwtStore } from '$lib/caching/stores/stores';
	// import Login from 'svelte-google-materialdesign-icons/Login.svelte';

	const drawerStore = getDrawerStore();

	let email = '';
	let storeEmailAnonymously = false;

	// const verificationMail = get(verificationMailStore);
	// if (verificationMail != null) {
	// 	username_or_email = verificationMail;
	// }

	let pwd = '';
	let invalidCreds = '';
	let sent = false;
	let success = false;
	let done = false;
	let error = '';
	let otp_done = false;

	let isLogin = true;

	const onLogin = async () => {
		success = true;
		error = '';
		sent = true;

		// client side checking of emails
		const jwt: JWTInfo | HasError | LoginResponseError = await login(
			email,
			pwd,
			storeEmailAnonymously
		);
		console.log(jwt);
		if (isLoginResponseError(jwt)) {
			if (jwt.invalid_email) {
				error = 'Felerhafte E-Mail-Adresse';
			} else if (jwt.missing_email) {
				error = 'E-Mail-Adresse fehlt';
			}
			success = false;
		} else if (isHasError(jwt)) {
			if (jwt.error.includes('OTP')) {
				error = 'Fehlerhaftes One-Time Passwort';
				success = true;
				otp_done = true;
			} else {
				error = 'Ein serverseitiger Fehler ist aufgetreten. Es kann nicht fortgefahren werden.';
			}
			success = false;
		} else {
			if (jwt.access_token.length > 0) {
				jwtStore.set(jwt.access_token);
				console.log(get(jwtStore));
				drawerStore.close();
				gotoHistory('/user');
			}
		}

		done = true;
		if (success) {
			otp_done = true;
		}
	};
</script>

<div class=" z-[10000] login_container flex flex-col text-black">
	<button
		on:click={() => {
			drawerStore.close();
		}}
		style="font-size: 27px"
		class="w-5 font-bold unselectable">&#x2715</button
	>
	<h2 class="text-center mb-4">
		{#if isLogin}
			Anmeldung
		{:else}
			Registrierung
		{/if}
	</h2>
	<label for="username">E-Mail</label>
	<div class="flex">
		<input id="username" placeholder="''dergertrud@gmail.com'" type="text" bind:value={email} />
	</div>

	{#if !isLogin}
		<label class="ml-2 flex items-center space-x-2">
			<input class="checkbox" type="checkbox" bind:checked={storeEmailAnonymously} />
			<p>E-Mail anonymisiert abspeichern</p>
		</label>
	{/if}

	{#if success}
		{#if sent && done}
			An die angegebene E-Mail-Addresse wurde eine Mail mit One-Time Passwort versendet.
		{/if}
	{/if}
	{#if otp_done}
		<label class="mt-4" for="password">One-Time Passwort (OTP)</label>
		<input id="password" placeholder="'MAS DS5 4DA'" type="password" bind:value={pwd} />
	{/if}

	{#if sent && !success}
		{error}
	{/if}

	<!-- <input class="mt-4" type="button" value="" on:click={onLogin} /> -->
	<SButton on:click={onLogin} class=" mt-4 bg-tertiary-500"
		>{isLogin ? 'Anmelden' : 'Registrieren'}</SButton
	>
	<span class="text-red-500">{invalidCreds}</span>

	<div class="flex justify-between">
		<div></div>
		<div>
			oder
			<!-- blue, underlined hovering stuff -->
			<span
				class="text-blue-500 unselectable underline"
				role="button"
				on:click={() => (isLogin = !isLogin)}
				on:keydown={() => (isLogin = !isLogin)}
				tabindex="0"
			>
				{#if isLogin}registrieren{:else}anmelden{/if}
			</span>
		</div>
	</div>
</div>

<style>
	input[type='text'],
	input[type='password'] {
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
		width: 500px;
		max-width: 95%;
		padding: 20px;
		border-radius: 10px;
		box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.2);
	}

	input[type='button'] {
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

	input[type='button']:hover {
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
