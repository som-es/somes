<script lang="ts">
	import { onMount } from 'svelte';
	import somesIcon from '$lib/assets/somes_icon.svg?raw';
	import { PLAY_STORE_URL } from '$lib/apps';
	import { t } from '$lib/i18n/i18n.svelte';

	const STORAGE_KEY = 'androidAppBannerDismissed';

	let visible = $state(false);

	onMount(() => {
		if (!/Android/i.test(navigator.userAgent)) return;
		try {
			visible = localStorage.getItem(STORAGE_KEY) !== 'true';
		} catch {
			visible = true;
		}
	});

	function dismiss() {
		visible = false;
		try {
			localStorage.setItem(STORAGE_KEY, 'true');
		} catch {
			return;
		}
	}
</script>

{#if visible}
	<div
		class="fixed inset-x-3 z-[60] flex items-center gap-3 rounded-xl bg-surface-500 p-3 text-white shadow-xl"
		style="bottom: calc(0.75rem + env(safe-area-inset-bottom, 0px));"
	>
		<span class="w-7 shrink-0 fill-white stroke-white">{@html somesIcon}</span>
		<span class="flex-1 text-sm">{t('app.banner.text')}</span>
		<a
			href={PLAY_STORE_URL}
			target="_blank"
			rel="noopener noreferrer"
			class="shrink-0 rounded-full bg-tertiary-400 px-4 py-1.5 text-sm font-bold text-black"
			onclick={dismiss}
		>
			{t('app.banner.open')}
		</a>
		<button
			type="button"
			class="shrink-0 px-1 text-xl leading-none"
			aria-label={t('app.banner.close')}
			onclick={dismiss}
		>
			×
		</button>
	</div>
{/if}
