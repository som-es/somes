<script lang="ts">
    import { delegates } from '$lib/api';
	import { onMount } from "svelte";
	import type { Delegate, VoteResult } from "$lib/types";
	import VoteParliament from '../VoteParliament.svelte';
	import { get, type Readable } from 'svelte/store';
	import { localStorageStore } from '@skeletonlabs/skeleton';
	
    let dels: Delegate[];
    
    onMount(async function () {
        const austrianDelegates = await delegates();
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");
    });

    const currentLegisInitStorage: Readable<VoteResult | null> = localStorageStore('selectedVoteResult', null);
    let voteResult: VoteResult | null = get(currentLegisInitStorage);
</script>

<div class="flex flex-col">
    {#if dels && voteResult}
        <div class="self-center">
            <h3>{voteResult.legislative_initiative.title}</h3>
            <h5>Akzeptiert: {voteResult.legislative_initiative.accepted ? "Ja" : "Nein"}</h5>
        </div>
        <div class="self-center m-auto w-7/12">
            <VoteParliament 
                dels={dels} 
                seats={[20, 27, 37, 43, 48, 54]} 
                voteResult={voteResult}
            />
        </div>
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>