<script lang="ts">
	import { isHasError } from '$lib/api/api';
	import { renew_token } from '$lib/api/authed';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';
	import { onMount } from 'svelte';

	onMount(async () => {
		if (!jwtStore.value) return;

		const renewedToken = await renew_token();
		if (isHasError(renewedToken)) {
			jwtStore.value = null;
		} else {
			jwtStore.value = renewedToken.access_token;
		}
	});
</script>
