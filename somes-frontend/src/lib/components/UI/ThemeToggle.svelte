<script lang="ts">
	import sunIcon from '$lib/assets/icons/sun.svg?raw';
	import moonIcon from '$lib/assets/icons/moon.svg?raw';
	import { lightModeStore, toggleTheme } from '$lib/lightmode.svelte';
	import { t } from '$lib/i18n/i18n.svelte';

	let { class: className = '' }: { class?: string } = $props();

	let isDark = $derived(lightModeStore.value === 'dark');
</script>

<button
	onclick={toggleTheme}
	class="flex cursor-pointer items-center gap-2 {className}"
	aria-pressed={isDark}
	title={isDark ? t('nav.menu.lightMode') : t('nav.menu.darkMode')}
>
	<span>{isDark ? t('nav.menu.lightMode') : t('nav.menu.darkMode')}</span>
	<span class="size-5 [&_svg]:h-full [&_svg]:w-full" aria-hidden="true">
		{@html isDark ? sunIcon : moonIcon}
	</span>
</button>
