<script lang="ts">
    import { delegates, latest_legis_inits_and_votes } from '$lib/api';
	import { onMount } from "svelte";
	import type { Delegate, LegislativeInitiative, VoteResult } from "$lib/types";
	import VoteParliament from '../VoteParliament.svelte';
	import { LightSwitch } from '@skeletonlabs/skeleton';

    let dels: Delegate[];
    let voteResults: VoteResult[];
    
    onMount(async function () {
        const austrianDelegates = await delegates();
        // use local storage to cache the delegates
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");

        voteResults = await latest_legis_inits_and_votes();

    });    
</script>

<div>
    {#if dels && voteResults}
        <VoteParliament 
            dels={dels} 
            seats={[20, 27, 37, 43, 48, 54]} 
            voteResult={voteResults[1]} 
        />
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>