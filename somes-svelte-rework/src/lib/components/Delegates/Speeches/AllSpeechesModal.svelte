<script lang="ts">
	import { getModalStore, type PopupSettings } from "@skeletonlabs/skeleton";
	import type { GovProposal, Speech } from "$lib/types";
	import Pagination from "$lib/components/Pagination.svelte";
	import SpeechBar from "./SpeechBar.svelte";
	import { errorToNull, speeches_by_delegate_per_page } from "$lib/api";
	import ExpandablePlaceholder from "$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";

	const modalStore = getModalStore();
	export let parent;


    let filteredSpeeches: Speech[] = [];
    let currentPageSpeeches: Speech[] = [];

    $: if ($modalStore.length > 0 && $modalStore[0].meta) {
        currentPageSpeeches = $modalStore[0].meta.speechesPage0.speeches;
    }

    let page = 1;
    
    $: if (page && $modalStore.length > 0 && $modalStore[0].meta) {
        currentPageSpeeches = []
        speeches_by_delegate_per_page($modalStore[0].meta.delegateId, page).then((res) => {
            currentPageSpeeches = (errorToNull(res)?.speeches ?? [])
        });
    }

    // $: speeches = 

</script>


{#if $modalStore.length > 0 && $modalStore[0].meta }
    <div class="card p-8 max-w-7xl">
        <button
            on:click={() => {
                modalStore.close();
            }}
            style="font-size: 34px"
            class="w-5 unselectable"
        >
            &#x2715
        </button>

        <Pagination bind:page maxPage={$modalStore[0].meta.speechesPage0.max_page -1} />
        <!-- <AllProposalsFiltering bind:filteredGovProposals={filteredGovProposals} allGovProposals={$modalStore[0].meta.govProposals} /> -->
        {#each currentPageSpeeches as speech}
            <SpeechBar {speech} />
            <!-- <GovProposalExpandableBar {govProposal} /> -->
        {/each}
        {#if currentPageSpeeches.length == 0}
            {#each { length: 15 } as _}
                <ExpandablePlaceholder class="min-w-7xl w-7xl" />
            {/each}
        {/if}
    </div>
{/if}