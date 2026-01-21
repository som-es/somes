<script lang="ts">
	import AllProposalsFiltering from './AllProposalsFiltering.svelte';
	import type { GovProposal } from '$lib/types';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';

	// const modalStore = getModalStore();
	// export let parent;

	let filteredGovProposals: GovProposal[] = [];
</script>

<div class="card p-8 max-w-7xl">
	<h1 class="font-bold text-2xl">Ministerialentwürfe</h1>
	<!-- <button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right"
	>
		✕
	</button> -->
<!-- 
	{#if $modalStore.length > 0 && $modalStore[0].meta}
		<AllProposalsFiltering
			bind:filteredGovProposals
			allGovProposals={$modalStore[0].meta.govProposals}
		/>
		{#each filteredGovProposals as govProposal}
			<GovProposalExpandableBar
				govProposal={{ gov_proposal: govProposal, delegate: $modalStore[0].meta.delegate }}
				coloring={'dark:bg-primary-300 bg-primary-400 text-black'}
			/>
		{/each}
	{/if} -->
</div>
