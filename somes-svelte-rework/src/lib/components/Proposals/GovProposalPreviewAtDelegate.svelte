<script lang="ts">
	import type { GovProposal } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import GovProposalExpandableBar from './ExpandableAtDelegate/GovProposalExpandableBar.svelte';

	export let govProposals: GovProposal[];

	$: allProposals = {
		type: 'component',
		component: 'allGovProposals',
		meta: { govProposals: govProposals }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewGovProposals = govProposals.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-2xl">Ministerialentwürfe</h1>

		<h2 class="text-lg">
			{govProposals.length} {govProposals.length == 1 ? "Ministerialentwurf" : "Ministerialentwürfe"} insgesamt
		</h2>
	</div>
	<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(allProposals)}>Alle anzeigen</button>
</div>
{#each previewGovProposals as govProposal}
	<GovProposalExpandableBar {govProposal} />
{/each}
