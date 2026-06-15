<script lang="ts">
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { cachedPartyColors } from '$lib/caching/party_color';
	import { cachedPlenarySessions } from '$lib/caching/plenarySessions';
	import { cachedAllSeats } from '$lib/caching/seats';
	import { setPartyColors } from '$lib/partyColor';
	import { onMount } from 'svelte';

	onMount(async function () {
		setPartyColors(await cachedPartyColors(true));
		await cachedAllLegisPeriods(true);
		await cachedAllSeats(true);
		await cachedPlenarySessions(true);
	});

	setInterval(
		async () => {
			setPartyColors(await cachedPartyColors(true));
			await cachedPlenarySessions(true);
		},
		1000 * 60 * 2
	);
	setInterval(async () => {}, 1000 * 60);
</script>
