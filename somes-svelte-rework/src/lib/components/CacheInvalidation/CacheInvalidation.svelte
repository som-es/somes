<script lang="ts">
	import { cachedDelegates } from '$lib/caching/delegates';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { cachedPartyColors } from '$lib/caching/party_color';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import { onMount } from 'svelte';

	onMount(async function () {
		await cachedPartyColors(true);
		await cachedAllLegisPeriods(true);
	});

	setInterval(
		async () => {
			cachedPartyColors(true);
			cachedDelegates(true);
		},
		1000 * 60 * 2
	);
	setInterval(
		async () => {
			cachedLatestVoteResults(true);
		},
		1000 * 60 * 4
	);
</script>
