<script lang="ts">
	import { delegates, latest_vote_results } from "$lib/api";
	import VoteResultExpandableBar from "$lib/components/VoteResults/VoteResultExpandableBar.svelte";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";


    let voteResults: VoteResult[] | null = null;

    let dels: Delegate[];
    onMount(async function () {
		voteResults = (await latest_vote_results());
		const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
	});
</script>

<div class="container mx-auto px-5">
    <h1>Vergangene Abstimmungsergebnisse</h1>
    {#if voteResults && dels}
        <div class="gap-3 mt-5">
            <VoteResultExpandableBar {dels} voteResult={voteResults[0]} class="w-5/6" />
        </div>
        <div class="gap-3 mt-5">
            <VoteResultExpandableBar {dels} voteResult={voteResults[1]} class="w-5/6" />
        </div>
        <div class="flex flex-col items-center gap-3 mt-1">
        </div>
    {/if}
</div>