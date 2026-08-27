<script lang="ts">
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import sunIcon from '$lib/assets/icons/sun.svg?raw';
	import moonIcon from '$lib/assets/icons/moon.svg?raw';
	import { page } from '$app/state';
	import { localeStore, setLocale, type Locale } from '$lib/i18n/i18n.svelte';
	import { lightModeStore } from '$lib/lightmode.svelte';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';
	import { accountOrLogin } from './user';
	import { t } from '$lib/i18n/i18n.svelte';
	import { Popover } from 'bits-ui';

	let activeUrl = $derived(page.url.pathname);
	const locales: Locale[] = ['de', 'en'];

	$effect(() => {
		const wantsDark =
			lightModeStore.value === 'dark' ||
			(!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches);

		if (wantsDark) {
			lightModeStore.value = 'dark';
		} else {
			lightModeStore.value = 'light';
		}
		document.documentElement.classList.toggle('dark', lightModeStore.value == 'dark' || wantsDark);
		document.cookie = `theme=${lightModeStore.value}; path=/; expires=${new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toUTCString()}`;
	});

	function toggleTheme() {
		lightModeStore.value = lightModeStore.value == 'light' ? 'dark' : 'light';
		localStorage.setItem('theme', lightModeStore.value);
	}
</script>

<Popover.Root>
	<Popover.Trigger openOnHover openDelay={10}>
		<button
			onclick={async () => {
				await accountOrLogin();
			}}
			title={t('nav.profile')}
			class="{activeUrl?.includes('/user')
				? 'bg-tertiary-500! fill-black'
				: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
		>
			<span class="h-5 w-5">
				{@html userIcon}
			</span>
		</button>
		<Popover.Portal>
			<Popover.Content>
				<div
					class="w-60 rounded-xl border border-gray-200 bg-white p-2 text-gray-900 shadow-xl dark:border-surface-500 dark:bg-surface-600 dark:text-gray-50"
				>
					<button
						onclick={toggleTheme}
						class="flex w-full cursor-pointer items-center justify-between gap-2 rounded-lg px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-surface-500"
					>
						<span>
							{lightModeStore.value == 'dark' ? t('nav.menu.lightMode') : t('nav.menu.darkMode')}
						</span>
						<span class="size-5 [&_svg]:h-full [&_svg]:w-full" aria-hidden="true">
							{@html lightModeStore.value == 'dark' ? sunIcon : moonIcon}
						</span>
					</button>

					<div class="px-3 py-2">
						<p class="mb-1.5 text-xs">{t('nav.menu.language')}</p>
						<div class="flex w-full gap-1 rounded-lg bg-gray-100 p-0.5 dark:bg-surface-700">
							{#each locales as locale}
								<button
									class="flex flex-1 cursor-pointer items-center justify-center rounded-md px-2 py-1 text-xs font-medium {localeStore.value ===
									locale
										? 'bg-primary-600 text-white'
										: 'text-gray-500 hover:bg-gray-200 dark:text-gray-400 dark:hover:bg-surface-500'}"
									onclick={() => setLocale(locale)}
								>
									{locale.toUpperCase()}
								</button>
							{/each}
						</div>
					</div>

					<hr class="my-1 border-gray-200 dark:border-surface-500" />

					<button
						onclick={async () => {
							await accountOrLogin();
						}}
						class="flex w-full cursor-pointer items-center justify-between gap-2 rounded-lg px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-surface-500"
					>
						<span>{jwtStore.value ? t('nav.profile') : t('login.title.signIn')}</span>
						<span class="size-4 fill-current">
							{@html userIcon}
						</span>
					</button>
				</div>
			</Popover.Content>
		</Popover.Portal>
	</Popover.Trigger>
</Popover.Root>
