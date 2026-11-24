<script lang="ts">
	import appStoreDe from '$lib/assets/badges/app-store-de.svg';
	import appStoreEn from '$lib/assets/badges/app-store-en.svg';
	import googlePlayDe from '$lib/assets/badges/google-play-de.png';
	import googlePlayEn from '$lib/assets/badges/google-play-en.png';
	import { APP_STORE_URL, PLAY_STORE_URL } from '$lib/apps';
	import { getLocale } from '$lib/i18n/i18n.svelte';

	let { class: clazz = '' }: { class?: string } = $props();

	let german = $derived(getLocale() === 'de');
</script>

<div class="flex flex-wrap items-center gap-3 {clazz}">
	<a href={APP_STORE_URL} target="_blank" rel="noopener noreferrer">
		<img src={german ? appStoreDe : appStoreEn} alt="App Store" class="h-10 w-auto" />
	</a>
	<a href={PLAY_STORE_URL} target="_blank" rel="noopener noreferrer">
		<img src={german ? googlePlayDe : googlePlayEn} alt="Google Play" class="h-10 w-auto" />
	</a>
</div>
