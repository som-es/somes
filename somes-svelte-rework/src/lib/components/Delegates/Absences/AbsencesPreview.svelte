<script lang="ts">
	import type { Absence, Speech, SpeechesWithMaxPage } from '$lib/types';
	import { type ModalSettings } from '@skeletonlabs/skeleton-svelte';
	import AbsenceBar from './AbsenceBar.svelte';

	export let absences: Absence[];
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
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte Abwesenheiten</h1>

		<h2 class="sm:text-lg">
			{absences.length}
			{absences.length == 1 ? 'Abwesenheit' : 'Abwesenheiten'} insgesamt
		</h2>
	</div>
	<button class="btn sm:btn-lg preset-filled mt-1" on:click={() => modalStore.trigger(allAbsences)}
		>Alle anzeigen</button
	>
</div>

<div class="mt-1">
	{#each previewSpeeches as absence}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<AbsenceBar {absence} page={0}></AbsenceBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
