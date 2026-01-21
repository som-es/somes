<script lang="ts">
	import type { Delegate, GovProposal } from '$lib/types';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';

	export let govProposals: GovProposal[];
	export let delegate: Delegate;

	// $: allProposals = {
	// 	type: 'component',
	// 	component: 'allGovProposals',
	// 	meta: { govProposals: govProposals, delegate }
	// } as ModalSettings;

	// const modalStore = getModalStore();

	$: previewGovProposals = govProposals.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Ministerialentwürfe</h1>

		<h2 class="sm:text-lg">
			{govProposals.length}
			{govProposals.length == 1 ? 'Ministerialentwurf' : 'Ministerialentwürfe'} insgesamt
		</h2>
	</div>
	<button
		class="btn sm:btn-lg preset-filled mt-1"
		on:click={() => {}/*modalStore.trigger(allProposals)*/}>Alle anzeigen</button
	>
</div>
{#each previewGovProposals as govProposal}
	<GovProposalExpandableBar
		govProposal={{ gov_proposal: govProposal, delegate }}
		coloring={'dark:bg-primary-300 bg-primary-400 text-black'}
	/>
{/each}
