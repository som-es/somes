<script lang="ts">
	import { errorToNull, vote_result_by_id } from "$lib/api";
	import VoteParliament2 from "$lib/components/Parliaments/VoteParliament2.svelte";
	import type { Speech, VoteResult } from "$lib/types";
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { onMount } from "svelte";
	import { gotoHistory } from "$lib/goto";

    export let speech: Speech;

    let voteResult: VoteResult | null = null;

    onMount(async () => {
        voteResult = errorToNull(await vote_result_by_id(speech.legislative_initiatives_id.toString()));

    }) 

	function onShowDetails(voteResult: VoteResult | null) {
		currentVoteResultStore.set(voteResult);
		gotoHistory('/vote_result', true);
	}

    $: opinion = speech.infavor != null ? (speech.infavor ? "Dafür gesprochen" : "Dagegen gesprochen") : speech.opinion 

</script>

<div class="gap-3 mt-5">
    <div
		class="entry dark:bg-primary-300 bg-primary-400 flex justify-between items-center text-black"
    >
        {#if voteResult}

            <div class="flex flex-col">
                <div class="text-lg font-bold">{opinion}</div>
                <div>{voteResult.legislative_initiative.description}</div>
            </div>

            {#if voteResult.votes.length > 0}
                <button
                    class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
                    on:click={() => onShowDetails(voteResult)}
                >
                    <VoteParliament2 voteResult={voteResult} preview={true} />
                </button>
            {/if}
        {:else}
            <div></div>
        {/if}
<!-- 
	<div use:collapse={{ open, duration }}>
		<GovProposalExpanded {govProposal} bind:open />
	</div> -->
    </div>
</div>
<!-- 
<div class="gap-3 rounded variant-filled my-1">
    {#if voteResult}
        {voteResult.legislative_initiative.description}
        {speech.legislative_initiatives_id} {speech.opinion} 
        {#if voteResult.votes.length > 0}
            <div>
                <VoteParliament2 {voteResult}></VoteParliament2>
            </div>
        {/if}
    {/if}
</div> -->

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>
