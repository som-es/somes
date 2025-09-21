<script lang="ts">
	import {
		unfinished_vote_results_by_search,
		unfinished_vote_results_per_page
	} from '$lib/api/api';
	import { cachedDelegates, filteredDelegates } from '$lib/caching/delegates';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';

	let dels: Delegate[] | null = null;
	onMount(async function () {
		dels = (await filteredDelegates())?.nr ?? null;
	});
</script>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Anträge zur Abstimmung</h1>

	{#if dels}
		<PaginationVoteResults
			{dels}
			storeIdx={1}
			voteResultsPostFn={unfinished_vote_results_per_page}
			voteResultsSearchPostFn={unfinished_vote_results_by_search}
			showAcceptedFilter={false}
			showVoteTypeFilter={false}
		/>
	{/if}
</Container>
<!-- </div> -->
