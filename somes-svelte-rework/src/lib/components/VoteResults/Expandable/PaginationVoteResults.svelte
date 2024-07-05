<script lang="ts">
	import type { Delegate, VoteResult } from "$lib/types";
	import { onMount } from "svelte";
	import { vote_results_per_page } from "$lib/api";
	import SButton from "../../UI/SButton.svelte";
	import VoteResultExpandableBar from "./VoteResultExpandableBar.svelte";

    export let dels: Delegate[];

    let voteResults: VoteResult[] | null = null;
    let page = 0;

    const loadVoteResults = async () => {
        voteResults = await vote_results_per_page(page);
    }
    onMount(async () => {
        await loadVoteResults();
    });

    $: loadVoteResults();


</script>

<div>
    {#if voteResults}
        {#each voteResults as voteResult}
            <VoteResultExpandableBar {dels} voteResult={voteResult} class="" />
        {/each}
    {/if}
    <div class="float-right">
        <SButton class="mt-5 mb-5 bg-tertiary-500" on:click={() => {page++; loadVoteResults()}}>Next</SButton>  
    </div>
</div>
<style>

</style>