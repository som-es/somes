<script lang="ts">
	import type { Speech } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import SpeechBar from './SpeechBar.svelte';

	export let speechesPage0: Speech[];

	$: allSpeeches = {
		type: 'component',
		component: 'allSpeeches',
		meta: { speechesPage0: speechesPage0 }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewSpeeches = speechesPage0.slice(0, 40);
</script>

<div class="flex flex-wrap justify-between items-center">
	<h1 class="font-bold text-2xl">Letzte Reden</h1>
	<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(allSpeeches)}>Alle anzeigen</button>
</div>
<div class="mt-5">
{#each previewSpeeches as speech}
    <!-- <div class="gap-3 rounded variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
	 <SpeechBar {speech}></SpeechBar>
	<!-- <GovProposalExpandableBar {govProposal} /> -->
{/each}

</div>
