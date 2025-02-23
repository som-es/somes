<script lang="ts">
	import type { Absence, Speech, SpeechesWithMaxPage } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import AbsenceBar from './AbsenceBar.svelte';

	export let absences: Absence[];
	console.log(absences);
	export let delegateId: number;

	$: allAbsences = {
		type: 'component',
		component: 'allAbsences',
		meta: { delegateId: delegateId, absences }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewSpeeches = absences.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<h1 class="font-bold text-2xl">Letzte Abwesenheiten</h1>
	<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(allAbsences)}>Alle anzeigen</button>
</div>

<div class="mt-5">
	{#each previewSpeeches as absence}
		<!-- <div class="gap-3 rounded variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<AbsenceBar {absence}></AbsenceBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>

