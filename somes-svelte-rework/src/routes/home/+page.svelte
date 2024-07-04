<script lang="ts">
	import { delegates, latest_vote_results } from "$lib/api";
	import VoteParliament from "$lib/components/Parliaments/VoteParliament.svelte";
	import { updateColorStorage } from "$lib/partyColor";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";

    let dels: Delegate[];
    let voteResults: VoteResult[];
    onMount(async function () {
		await updateColorStorage();

		const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

		voteResults = (await latest_vote_results()).slice(0, 6);
	}); 

</script>
<div class="container mx-auto px-0 mt-5">
    home
    {#if voteResults}
        {#each voteResults as voteResult}
            <div>
                <h1>{voteResult.legislative_initiative.created_at}</h1>
                <VoteParliament {dels} {voteResult} preview={true}/>
            </div>
        {/each}
    {/if}
</div>