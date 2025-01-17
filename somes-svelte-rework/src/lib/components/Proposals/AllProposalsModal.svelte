<script lang="ts">
	import { getModalStore, type PopupSettings } from "@skeletonlabs/skeleton";
	import GovProposalExpandableBar from "./Expandable/GovProposalExpandableBar.svelte";
	import AllProposalsFiltering from "./AllProposalsFiltering.svelte";
	import type { GovProposal } from "$lib/types";

	const modalStore = getModalStore();
	export let parent;


    let filteredGovProposals: GovProposal[] = [];

</script>

<div class="card p-8 max-w-7xl">
    <button
        on:click={() => {
            modalStore.close();
        }}
        style="font-size: 34px"
        class="w-5 unselectable float-right"
    >
        &#x2715
    </button>

    {#if $modalStore.length > 0 && $modalStore[0].meta }
        <AllProposalsFiltering bind:filteredGovProposals={filteredGovProposals} allGovProposals={$modalStore[0].meta.govProposals} />
        {#each filteredGovProposals as govProposal}
            <GovProposalExpandableBar {govProposal} />
        {/each}
    {/if}
</div>