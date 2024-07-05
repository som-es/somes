<script lang="ts">
	import type { Delegate, VoteResult, VoteResultsWithMaxPage } from "$lib/types";
	import { onMount } from "svelte";
	import { vote_results_per_page } from "$lib/api";
	import SButton from "../../UI/SButton.svelte";
	import VoteResultExpandableBar from "./VoteResultExpandableBar.svelte";
	import { pushState } from "$app/navigation";
	import Pagination from "$lib/components/Pagination.svelte";

    export let dels: Delegate[];

    let voteResults: VoteResultsWithMaxPage | null = null;

    // get page number from query params
    const url = new URL(window.location.href);
    let page = parseInt(url.searchParams.get("page") || "1") || 1;
   
    const loadVoteResults = async () => {
        voteResults = await vote_results_per_page(page - 1);
    }
    onMount(async () => {
        update();
    });

    const update = () => {
        loadVoteResults();

        // update query params
        const url = new URL(window.location.href);
        url.searchParams.set("page", page.toString());
        pushState(url.toString(), { replaceState: true });

    }

    $: if (page) {
        update();
    }

</script>

<div>
    {#if voteResults}
            <Pagination bind:page={page} maxPage={voteResults.max_page} />
        {#each voteResults.vote_results as voteResult}
            <VoteResultExpandableBar {dels} voteResult={voteResult} class="" />
        {/each}
         <div class="float-right">
            <Pagination bind:page={page} maxPage={voteResults.max_page} />
        </div>
    {/if}
   
</div>
<style>

</style>