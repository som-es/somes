<script lang="ts">
	import type { GovProposal } from '$lib/types';
	import GovProposalExpandableBar from './Expandable/GovProposalExpandableBar.svelte';

	export let govProposals: GovProposal[];

	$: previewGovProposals = govProposals.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<h1 class="font-bold text-2xl">Ministerialentwürfe</h1>
	<button class="btn btn-lg variant-filled" on:click={() => ''}>Alle anzeigen</button>
</div>
{#each previewGovProposals as govProposal}
	<GovProposalExpandableBar {govProposal} />
{/each}
