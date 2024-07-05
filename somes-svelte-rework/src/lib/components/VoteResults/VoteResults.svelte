<script lang="ts">
	import { latest_vote_results } from "$lib/api";
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";
	import VoteResultComp from "./VoteResult.svelte";
    import collapse from 'svelte-collapse'
	import { goto } from "$app/navigation";
	import upArrowIcon from "$lib/assets/misc_icons/up-arrow.svg?raw";
	import downArrowIcon from "$lib/assets/misc_icons/down-arrow.svg?raw";
    
    export let dels: Delegate[];

    let voteResults: VoteResult[] | null = null;
    let firstThreeVotes: VoteResult[] = [];
    let restVotes: VoteResult[] = []

    onMount(async function () {
		voteResults = (await latest_vote_results());
        firstThreeVotes = voteResults.slice(0, 3);
        restVotes = voteResults.slice(3);
	});

    let open = false;
</script>

{#if voteResults}
    {#if voteResults.length == 0}
        <p class="no-votes">Keine Abstimmungsergebnisse</p>
    {/if}
    <div class="card-container">
        {#each firstThreeVotes as voteResult, i}
            <VoteResultComp {dels} {voteResult} tabindex={i}/>
        {/each}
        
    </div>
    <div class="flex justify-between px-3">
        <button class="expand-button bg-primary-500" on:click={() => open = !open}>
            {#if open}
                Weniger anzeigen
            {:else}
                Mehr anzeigen
            {/if}
        </button>
        <button class="expand-button bg-secondary-500" on:click={() => goto("legis_votes_history")}>
            Vorherige anzeigen
        </button>
    </div>
    <hr>

    <div use:collapse={{open}}>
        <div 
            on:click={() => open = !open} 
            on:keypress={() => open = !open}
            class="card-container z-0"
            role="button"
            tabindex="0"
        >

            {#each restVotes as voteResult, i}
                <VoteResultComp {dels} {voteResult} tabindex={i}/>
            {/each}
        </div>
    </div>
{/if}

<style>
    .expand-button {
        display: block;
        margin-top: 1rem;
        margin-bottom: 1rem;
        padding: 0.5rem 1rem;
        border: 1px solid #000;
        border-radius: 5px;
        cursor: pointer;
    }
    .card-container {
        margin: auto;
        display: flex;
        flex-direction: row;
        /* remove this maybe again */
        justify-content: space-between;
        flex-wrap: wrap;
    }
</style>