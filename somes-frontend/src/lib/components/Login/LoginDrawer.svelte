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
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import googleIcon from '$lib/assets/misc_icons/google.svg?raw';

	// Props
	let { open = $bindable(false) } = $props();

	const API_BASE = import.meta.env.VITE_API_URL

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

	type OAuthProvider = 'google' | 'github' | 'discord' | 'microsoft';

	// OAuth Buttons
	const providers = [
		{ name: 'Google', key: 'google', color: 'bg-red-500', icon: 'G' },
		// { name: 'GitHub', key: 'github', color: 'bg-gray-800', icon: '🐱' },
		// { name: 'Discord', key: 'discord', color: 'bg-indigo-500', icon: 'D' },
		// { name: 'Microsoft', key: 'microsoft', color: 'bg-blue-600', icon: 'M' },
	];

	function startOAuth(providerKey: string) {
		window.location.href = `${API_BASE}/api/oauth/${providerKey}`;
	}
</script>

<Drawer.Root bind:open>
	<Drawer.Portal>
		<!-- Backdrop -->
		<Drawer.Overlay
			class="fixed inset-0 z-50 bg-black/40 backdrop-blur-sm data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>

		<!-- Drawer Content (Sliding from Right) -->
		<Drawer.Content
			class="fixed top-0 right-0 z-50 h-full w-[440px] max-w-[100vw] border-l border-gray-200 bg-white shadow-2xl transition ease-in-out data-[state=closed]:duration-200 data-[state=closed]:animate-out data-[state=closed]:slide-out-to-right data-[state=open]:duration-300 data-[state=open]:animate-in data-[state=open]:slide-in-from-right"
		>
			<div class="flex h-full flex-col">
				<!-- Header -->
				<div class="border-b border-gray-100 px-8 pt-6 pb-6">
					<Drawer.Close
						class="absolute top-5 right-5 flex h-8 w-8 items-center justify-center rounded-md text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
					>
						<span class="w-5 inline-block">{@html crossmarkIcon}</span>
					</Drawer.Close>

					<Drawer.Title class="text-2xl font-semibold text-gray-900">
						{#if isLogin}Anmelden{:else}Registrierung{/if}
					</Drawer.Title>
					<p class="mt-1 text-sm text-gray-500">
						{#if isLogin}
							Willkommen zurück. Gib deine E-Mail-Adresse ein.
						{:else}
							Erstelle ein neues Konto mit deiner E-Mail-Adresse.
						{/if}
					</p>
				</div>

				<!-- Form Body -->
				<div class="flex flex-col gap-5 px-8 py-7">
					<!-- Email Field -->
					<div class="flex flex-col gap-1.5">
						<label for="username" class="text-sm font-medium text-gray-700">E-Mail</label>
						<input
							id="username"
							placeholder="dergertrud@gmail.com"
							type="email"
							class="w-full rounded-lg border border-gray-200 bg-white px-3 py-2 text-base text-gray-900 outline-none transition placeholder:text-gray-400 focus:border-[rgb(104,129,161)] focus:shadow-[0_0_0_3px_rgba(104,129,161,0.15)]"
							onkeydown={onEnterDoLogin}
							bind:value={email}
						/>
					</div>

					{#if !isLogin}
						<div class="flex items-start gap-3 rounded-lg border border-gray-200 bg-gray-50 p-3">
							<Switch.Root
							bind:checked={storeEmailAnonymously}
							class="mt-0.5 inline-flex h-[22px] w-[40px] shrink-0 cursor-pointer touch-manipulation items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary-500 data-[state=unchecked]:bg-gray-300"
								id="storeEmailAnonymously"
							>
								<Switch.Thumb
									class="pointer-events-none block h-4 w-4 rounded-full bg-white shadow ring-0 transition-transform data-[state=checked]:translate-x-[18px] data-[state=unchecked]:translate-x-0"
								/>
							</Switch.Root>
							<label class="cursor-pointer" for="storeEmailAnonymously">
								<span class="block text-sm font-medium text-gray-800">E-Mail anonymisiert speichern</span>
								<span class="block text-xs text-gray-500 mt-0.5">
									Optionale E-Mail-Benachrichtigungen zu Abstimmungen sind dann nicht möglich.
								</span>
							</label>
						</div>
					{/if}

					{#if success && sent && done}
						<div class="rounded-lg border border-green-200 bg-green-50 px-4 py-3 text-sm text-green-700">
							An deine E-Mail-Adresse wurde ein One-Time Passwort gesendet.
						</div>
					{/if}

					{#if otp_done}
						<div class="flex flex-col gap-1.5">
							<label class="text-sm font-medium text-gray-700" for="password">One-Time Passwort (OTP)</label>
							<input
								id="password"
								placeholder="MAS DS5 4DA"
								type="password"
								class="w-full rounded-lg border border-gray-200 bg-white px-3 py-2 text-base text-gray-900 outline-none transition placeholder:text-gray-400 focus:border-[rgb(104,129,161)] focus:shadow-[0_0_0_3px_rgba(104,129,161,0.15)]"
								onkeydown={onEnterDoLogin}
								bind:value={pwd}
							/>
						</div>
					{/if}

					{#if sent && !success}
						<p class="text-sm text-red-500">{error}</p>
					{/if}

					<!-- Primary Action Button -->
					<button
						onclick={onLogin}
						class="w-full rounded-lg bg-secondary-500 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:brightness-110 active:scale-[0.98]"
					>
						{isLogin ? 'Anmelden' : 'Registrieren'}
					</button>

					<!-- Toggle mode -->
					<p class="text-center text-sm text-gray-500">
						{isLogin ? 'Noch kein Konto?' : 'Bereits registriert?'}
						<button
							type="button"
							class="ml-1 font-medium text-primary-600 hover:text-primary-800 hover:underline bg-transparent border-none p-0"
							onclick={toggleMode}
						>
							{#if isLogin}Registrieren{:else}Anmelden{/if}
						</button>
					</p>

					<!-- Divider -->
					<div class="flex items-center gap-3">
						<div class="h-px flex-1 bg-gray-200"></div>
						<span class="text-xs font-medium text-gray-400 uppercase tracking-wide">oder</span>
						<div class="h-px flex-1 bg-gray-200"></div>
					</div>

					<!-- Google OAuth Button -->
					<button
						class="flex w-full items-center justify-center gap-2.5 rounded-lg border border-gray-200 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 shadow-sm transition hover:bg-gray-50 hover:shadow active:bg-gray-100"
						onclick={() => startOAuth('google')}
					>
						{@html googleIcon}
						<span>Mit Google anmelden</span>
					</button>
				</div>
			</div>
		</Drawer.Content>
	</Drawer.Portal>
</Drawer.Root>
