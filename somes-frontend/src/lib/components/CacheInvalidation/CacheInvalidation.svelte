<script lang="ts">
	import { cachedDelegates } from '$lib/caching/delegates.svelte';
	import { cachedLatestGovProposals } from '$lib/caching/gov_proposals';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { cachedPartyColors } from '$lib/caching/party_color';
	import { cachedAllSeats } from '$lib/caching/seats';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import { setPartyColors } from '$lib/partyColor';
	import { onMount } from 'svelte';

	onMount(async function () {
		setPartyColors(await cachedPartyColors(true));
		await cachedAllLegisPeriods(true);
		await cachedLatestVoteResults(true);
		await cachedLatestGovProposals(true);
		await cachedAllSeats(true);
	});

	setInterval(
		async () => {
			setPartyColors(await cachedPartyColors(true));
			cachedDelegates(true);
		},
		1000 * 60 * 2
	);
	setInterval(async () => {
		cachedLatestVoteResults(true);
		cachedLatestGovProposals(true);
	}, 1000 * 60);
</script>
