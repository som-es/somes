<script lang="ts">
	import type { Speech } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

	export let speeches: Speech[];

	$: allSpeeches = {
		type: 'component',
		component: 'allSpeeches',
		meta: { speeches: speeches }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewSpeeches = speeches.slice(0, 4);
</script>

<div class="flex flex-wrap justify-between items-center">
	<h1 class="font-bold text-2xl">Letzte Reden</h1>
	<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(allSpeeches)}>Alle anzeigen</button>
</div>
{#each previewSpeeches as govProposal}
	<!-- <GovProposalExpandableBar {govProposal} /> -->
{/each}
