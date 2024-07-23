<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	let dels: Delegate[] | null = null;
	let voteResults: VoteResult[] | null = null;
	onMount(async function () {
		// await updateColorStorage();
		dels = await filteredDelegates();
		voteResults = await cachedLatestVoteResults();
	});
</script>

<Container>
	home
	{#if voteResults && dels}
		<VoteResults {dels} {voteResults} />
	{:else}
		<ProgressRadial />
	{/if}
</Container>
