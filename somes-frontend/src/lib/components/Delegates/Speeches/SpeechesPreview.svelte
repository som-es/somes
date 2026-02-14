<script lang="ts">
	import type { Speech, SpeechesWithMaxPage } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import AllSpeechesModal from './AllSpeechesModal.svelte';
	import SpeechBar from './SpeechBar.svelte';

	export let speechesPage0: SpeechesWithMaxPage;
	export let delegateId: number;

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
	<ExtendInfoDialog title="Alle anzeigen">
		<AllSpeechesModal {delegateId} {speechesPage0} />
	</ExtendInfoDialog>
</div>
<div class="mt-5">
	{#each previewSpeeches as speech}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<SpeechBar {speech}></SpeechBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
