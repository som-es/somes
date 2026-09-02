<script lang="ts">
	import { localeStore, setLocale, type Locale } from '$lib/i18n/i18n.svelte';

	let { dark = false, class: className = '' }: { dark?: boolean; class?: string } = $props();

	const locales: Locale[] = ['de', 'en'];

	const label: Record<Locale, string> = {
		de: 'DE',
		en: 'EN'
	};

	// Keep `localeStore.value` reactive in this component.
	let activeLocale = $derived(localeStore.value);

	const activeClass = $derived(
		dark
			? 'bg-secondary-500 text-white shadow'
			: 'bg-white text-surface-900 shadow dark:bg-secondary-500 dark:text-white'
	);
	const inactiveClass = $derived(
		dark
			? 'text-white/70 hover:bg-white/10 hover:text-white'
			: 'text-surface-500 hover:bg-surface-900/10 hover:text-surface-900 dark:text-surface-200 dark:hover:bg-white/10 dark:hover:text-white'
	);
</script>

<div
	class="flex items-center gap-1 rounded-xl p-1 text-xs font-bold {dark
		? 'bg-black/30'
		: 'bg-surface-900/10 dark:bg-surface-900/50'} {className}"
	title="Language / Sprache"
	role="group"
	aria-label="Language switcher"
>
	{#each locales as l}
		<button
			class="flex-1 cursor-pointer rounded-lg px-2 py-1 transition-colors {activeLocale === l
				? activeClass
				: inactiveClass}"
			aria-pressed={activeLocale === l}
			onclick={() => setLocale(l)}
		>
			{label[l]}
		</button>
	{/each}
</div>
