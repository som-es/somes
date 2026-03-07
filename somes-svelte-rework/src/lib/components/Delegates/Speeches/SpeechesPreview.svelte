<script lang="ts">
	import type { Speech, SpeechesWithMaxPage } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import SpeechBar from './SpeechBar.svelte';

	export let speechesPage0: SpeechesWithMaxPage;
	export let delegateId: number;

	$: allSpeeches = {
		type: 'component',
		component: 'allSpeeches',
		meta: { delegateId: delegateId, speechesPage0: speechesPage0 }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewSpeeches = speechesPage0.speeches.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte Reden</h1>

		<h2 class="sm:text-lg">
			{speechesPage0.entry_count}
			{speechesPage0.entry_count == 1 ? 'Rede' : 'Reden'} insgesamt
		</h2>
	</div>
	<button class="btn sm:btn-lg variant-filled mt-1" on:click={() => modalStore.trigger(allSpeeches)}
		>Alle anzeigen</button
	>
</div>
<div class="mt-5">
	{#each previewSpeeches as speech}
		<!-- <div class="gap-3 rounded variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<SpeechBar {speech}></SpeechBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
