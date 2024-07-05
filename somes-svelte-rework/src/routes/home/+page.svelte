<script lang="ts">
	import VoteResults from "$lib/components/VoteResults/VoteResults.svelte";
    import { cachedDelegates, filteredDelegates } from "$lib/caching/delegates";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";
	import { cachedPartyColors } from "$lib/caching/party_color";
	import { cachedLatestVoteResults } from "$lib/caching/vote_results";
	import SButton from "$lib/components/UI/SButton.svelte";

    let dels: Delegate[] | null = null;
    let voteResults: VoteResult[] | null = null;
    onMount(async function () {
        // await updateColorStorage();
        dels = await filteredDelegates();
        voteResults = await cachedLatestVoteResults();
	}); 

</script>
<div class="container mx-auto px-0 mt-5">
    home
    {#if voteResults && dels}
        <VoteResults {dels} {voteResults} />
    {:else}
        <p>loading...</p>
    {/if}
</div>