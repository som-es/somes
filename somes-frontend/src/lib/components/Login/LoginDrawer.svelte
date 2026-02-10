<script lang="ts">
	import type { JWTInfo, HasError, LoginResponseError } from '$lib/types';
	import { isHasError, isLoginResponseError } from '$lib/api/api';
	import { login } from '$lib/api/authed';

	// Bits UI Imports
	// We use Dialog primitives to build a Sidebar/Drawer
	import { Dialog as Drawer, Switch } from 'bits-ui';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	// Props
	let { open = $bindable(false) } = $props();

	// State
	let email = $state('');
	let storeEmailAnonymously = $state(false);
	let pwd = $state('');

	let invalidCreds = $state('');
	let sent = $state(false);
	let success = $state(false);
	let done = $state(false);
	let error = $state('');
	let otp_done = $state(false);
	let isLogin = $state(true);

	const onLogin = async () => {
		success = true;
		error = '';
		sent = true;

		// Client side checking of emails
		const jwt: JWTInfo | HasError | LoginResponseError = await login(
			email,
			pwd,
			storeEmailAnonymously
		);

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
				jwtStore.value = jwt.access_token;
				open = false; // Close drawer
				goto(resolve('/user'));
			}
		}

		done = true;
		if (success) {
			otp_done = true;
		}
	};

	const onEnterDoLogin = async (e: KeyboardEvent) => {
		if (e.code === 'Enter') {
			await onLogin();
		}
	};

	const toggleMode = () => {
		isLogin = !isLogin;
	};
</script>

<Drawer.Root bind:open>
	<Drawer.Portal>
		<!-- Backdrop -->
		<Drawer.Overlay
			class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>

		<!-- Drawer Content (Sliding from Right) -->
		<Drawer.Content
			class="fixed top-0 right-0 z-50 h-full w-[500px] max-w-[100vw] border-l bg-white p-6 shadow-2xl transition ease-in-out data-[state=closed]:duration-200 data-[state=closed]:animate-out data-[state=closed]:slide-out-to-right data-[state=open]:duration-300 data-[state=open]:animate-in data-[state=open]:slide-in-from-right sm:max-w-[500px]"
		>
			<div class="flex h-full flex-col text-black">
				<!-- Close Button -->
				<Drawer.Close
					class="ring-offset-background focus:ring-ring data-[state=open]:bg-secondary absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:pointer-events-none"
				>
					<span class="text-2xl font-bold">✕</span>
					<span class="sr-only">Close</span>
				</Drawer.Close>

				<Drawer.Title class="mb-6 text-center text-2xl font-bold">
					{#if isLogin}
						Anmelden
					{:else}
						Registrierung
					{/if}
				</Drawer.Title>

				<div class="mt-4 flex flex-col gap-4">
					<div>
						<label for="username" class="mb-2 block font-medium">E-Mail</label>
						<input
							id="username"
							placeholder="dergertrud@gmail.com"
							type="email"
							class="custom-input"
							onkeydown={onEnterDoLogin}
							bind:value={email}
						/>
					</div>

					{#if !isLogin}
						<div class="flex items-center gap-3">
							<Switch.Root
								bind:checked={storeEmailAnonymously}
								class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
								id="storeEmailAnonymously"

							>
								<Switch.Thumb
									class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
								/>
							</Switch.Root>
							<label
								class="cursor-pointer"
								for="storeEmailAnonymously"
							>
								<div class="flex flex-col">
									<span class="font-semibold ">
										E-Mail anonymisiert speichern
									</span>
									<span class=" text-sm">
										Achtung: Optionale E-Mail-Benachrichtungen zu Abstimmungen, etc. sind nicht möglich
									</span>
								</div>
							</label>
						</div>
					{/if}

					{#if success}
						{#if sent && done}
							<div
								class="rounded-md border border-green-200 bg-green-50 p-3 text-sm text-green-700"
							>
								An die angegebene E-Mail-Addresse wurde eine Mail mit One-Time Passwort versendet.
							</div>
						{/if}
					{/if}

					{#if otp_done}
						<div>
							<label class="mb-2 block font-medium" for="password">One-Time Passwort (OTP)</label>
							<input
								id="password"
								placeholder="MAS DS5 4DA"
								type="password"
								class="custom-input"
								onkeydown={onEnterDoLogin}
								bind:value={pwd}
							/>
						</div>
					{/if}

					{#if sent && !success}
						<div class="text-sm font-medium text-red-500">{error}</div>
					{/if}

					<!-- Action Button -->
					<button
						onclick={onLogin}
						class="mt-4 w-full rounded-md bg-tertiary-500 px-4 py-3 text-lg font-medium text-white shadow-md transition-all hover:brightness-110 active:scale-[0.98]"
					>
						{isLogin ? 'Anmelden' : 'Registrieren'}
					</button>

					<span class="text-center text-red-500">{invalidCreds}</span>

					<div class="mt-2 flex items-center justify-between">
						<div class="flex-grow"></div>
						<div class="text-sm text-gray-600">
							oder
							<button
								type="button"
								class="ml-1 cursor-pointer border-none bg-transparent p-0 text-blue-500 underline hover:text-blue-700"
								onclick={toggleMode}
							>
								{#if isLogin}registrieren{:else}anmelden{/if}
							</button>
						</div>
					</div>
				</div>
			</div>
		</Drawer.Content>
	</Drawer.Portal>
</Drawer.Root>

<style>
	/* Kept the specific shadow/border styles from original logic,
	   but moved generic layout properties to Tailwind above */
	.custom-input {
		width: 100%;
		padding: 10px;
		border: 1px solid #e5e7eb;
		border-radius: 5px;
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.05);
		font-size: 16px;
		outline: none;
		transition: box-shadow 0.2s;
	}

	.custom-input:focus {
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.15);
		border-color: #d1d5db;
	}
</style>
