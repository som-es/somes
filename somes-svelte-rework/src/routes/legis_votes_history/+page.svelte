<script lang="ts">
	import { delegates, latest_vote_results } from "$lib/api";
	import { cachedDelegates } from "$lib/caching/delegates";
	import VoteResultExpandableBar from "$lib/components/VoteResults/VoteResultExpandableBar.svelte";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";


    let voteResults: VoteResult[] | null = null;

    let dels: Delegate[];
    onMount(async function () {
		voteResults = (await latest_vote_results());
		const austrianDelegates = await cachedDelegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
	});
</script>

<div class="container mx-auto px-5">
    <h1>Vergangene Abstimmungsergebnisse</h1>
    {#if voteResults && dels}
        {#each voteResults as voteResult}
            <VoteResultExpandableBar {dels} voteResult={voteResult} class="w-5/6" />
        {/each}
    {/if}
</div>