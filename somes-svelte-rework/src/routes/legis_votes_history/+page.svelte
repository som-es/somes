<script lang="ts">
	import { delegates, latest_vote_results } from "$lib/api";
	import { cachedDelegates, filteredDelegates } from "$lib/caching/delegates";
	import PaginationVoteResults from "$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";


    let voteResults: VoteResult[] | null = null;

    let dels: Delegate[] | null = null;
    onMount(async function () {
		voteResults = (await latest_vote_results());
		dels = await filteredDelegates();
	});
</script>

<div class="container mx-auto px-5">
    <h1>Vergangene Abstimmungsergebnisse</h1>

    {#if dels}
        <PaginationVoteResults {dels} />
    {/if}
</div>