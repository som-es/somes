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
</script>

<div
	class="flex items-center gap-1 rounded-xl bg-surface-900/20 p-1 text-xs font-bold {className}"
	title="Language / Sprache"
	role="group"
	aria-label="Language switcher"
>
	{#each locales as l}
		<button
			class="flex-1 cursor-pointer rounded-lg px-2 py-1 transition-colors {activeLocale === l
				? dark
					? 'bg-secondary-500 text-white'
					: 'bg-white text-surface-900 shadow'
				: dark
					? 'text-white/60 hover:text-white'
					: 'text-surface-500 hover:text-surface-900'}"
			onclick={() => setLocale(l)}
		>
			{label[l]}
		</button>
	{/each}
</div>
