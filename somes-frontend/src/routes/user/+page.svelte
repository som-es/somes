<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';

	onMount(() => {
		const token = page.url.searchParams.get('token');
		if (token) {
			jwtStore.value = token;
			goto(resolve('/user'), { replaceState: true });
		}
	});
</script>
