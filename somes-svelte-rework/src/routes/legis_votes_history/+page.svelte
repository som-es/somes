<script lang="ts">
	import { cachedDelegates, filteredDelegates } from '$lib/caching/delegates';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';

	let dels: Delegate[] | null = null;
	onMount(async function () {
		dels = await filteredDelegates();
	});
</script>

<div class="mx-auto px-5">
	<h1 class="text-4xl font-bold">Vergangene Abstimmungsergebnisse</h1>

	{#if dels}
		<PaginationVoteResults {dels} />
	{/if}
</div>
