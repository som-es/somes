<script lang="ts">
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import { page } from '$app/state';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';
	import { accountOrLogin } from './user';
	import { t } from '$lib/i18n/i18n.svelte';
	import { Popover } from 'bits-ui';
	import ThemeToggle from '$lib/components/UI/ThemeToggle.svelte';
	import LanguageSwitcher from '$lib/components/UI/LanguageSwitcher.svelte';

	let activeUrl = $derived(page.url.pathname);
</script>

<Popover.Root>
	<Popover.Trigger
		openOnHover
		openDelay={10}
		onclick={async () => {
			await accountOrLogin();
		}}
		title={t('nav.profile')}
		class="{activeUrl?.includes('/user')
			? 'bg-tertiary-500! fill-black'
			: 'fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
	>
		<span class="h-5 w-5">
			{@html userIcon}
		</span>
	</Popover.Trigger>
	<Popover.Portal>
		<Popover.Content>
			<div
				class="w-60 rounded-xl border border-gray-200 bg-white p-2 text-gray-900 shadow-xl dark:border-surface-500 dark:bg-surface-600 dark:text-gray-50"
			>
				<ThemeToggle
					class="w-full justify-between rounded-lg px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-surface-500"
				/>

				<div class="px-3 py-2">
					<p class="mb-1.5 text-xs">{t('nav.menu.language')}</p>
					<LanguageSwitcher class="w-full" />
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
</Popover.Root>
