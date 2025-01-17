<script lang="ts">
	import { errorToNull, vote_result_by_id } from "$lib/api";
	import VoteParliament2 from "$lib/components/Parliaments/VoteParliament2.svelte";
	import type { Speech, VoteResult } from "$lib/types";
	import { currentVoteResultStore } from '$lib/stores/stores';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import { onMount } from "svelte";
	import { gotoHistory } from "$lib/goto";
	import { getModalStore } from "@skeletonlabs/skeleton";
	import ExpandablePlaceholder from "$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";

    export let speech: Speech;

    let voteResult: VoteResult | null = null;

    $: if(speech) {
        voteResult = null;
        vote_result_by_id(speech.legislative_initiatives_id.toString()).then((res) => {
            voteResult = errorToNull(res);
        });
    }

	const modalStore = getModalStore();

	function onShowDetails(voteResult: VoteResult | null) {
		currentVoteResultStore.set(voteResult);
        modalStore.close();
		gotoHistory('/vote_result', true);
        
	}

    $: opinion = speech.infavor != null ? (speech.infavor ? "Dafür gesprochen" : "Dagegen gesprochen") : speech.opinion 
    $: arrowBackground = (voteResult != null && voteResult.votes.length > 0) ? "bg-secondary-400" : "dark:bg-primary-300 bg-primary-400"
    $: hasVotes = (voteResult?.votes ?? []).length > 0

</script>

<div class="gap-3 mt-5">
    <div
		class="entry flex {arrowBackground} justify-between items-center text-black"
    >
        {#if voteResult}
            <div class="border-radius-left spacing-for-left flex dark:bg-primary-300 bg-primary-400 justify-between items-center flex-basis-left">
                <div class="flex flex-col">
                    <div class="text-lg font-bold">{opinion}</div>
                    <div>{voteResult.legislative_initiative.description}</div>
                </div>

                {#if hasVotes}
                    <button
                        class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
                        on:click={() => onShowDetails(voteResult)}
                    >
                        <VoteParliament2 voteResult={voteResult} preview={true} />
                    </button>
                {/if}
            </div> 

            {#if hasVotes}
                <button class="spacing-for-right" on:click={() => onShowDetails(voteResult)}>
                    {@html rightArrowIcon}
                </button>
            {/if}
        {:else}
            <ExpandablePlaceholder class="min-w-7xl w-7xl" />
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
	}

    .border-radius-left {
        border-top-left-radius: 0.9rem;
        border-bottom-left-radius: 0.9rem;
    }

    .spacing-for-left {
		padding: 20px;
		gap: 10px;
    }

    .spacing-for-right {
		padding: 20px;
		gap: 10px;
    }

    .flex-basis-left {
        flex-basis: 96%;
    }
</style>
