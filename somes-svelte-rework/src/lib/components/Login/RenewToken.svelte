<script lang="ts">
	import { isHasError, renew_token } from '$lib/api';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	onMount(async () => {
		if (!get(jwtStore)) return;

		const renewedToken = await renew_token();
		if (isHasError(renewedToken)) {
			jwtStore.set(null);
		} else {
			jwtStore.set(renewedToken.access_token);
		}
	});
</script>
