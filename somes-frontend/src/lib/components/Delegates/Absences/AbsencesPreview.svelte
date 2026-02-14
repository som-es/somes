<script lang="ts">
	import type { Absence, Speech, SpeechesWithMaxPage } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import AbsenceBar from './AbsenceBar.svelte';
	import AbsencesModal from './AbsencesModal.svelte';

	interface Props {
		absences: Absence[];
		delegateId: number;
	}

	let { absences, delegateId }: Props = $props();

	let previewSpeeches = $derived(absences.slice(0, 2));
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte Abwesenheiten</h1>

		<h2 class="sm:text-lg">
			{absences.length}
			{absences.length == 1 ? 'Abwesenheit' : 'Abwesenheiten'} insgesamt
		</h2>
	</div>
	<ExtendInfoDialog title="Alle anzeigen">
		<AbsencesModal absences={absences} />
	</ExtendInfoDialog>
</div>

<div class="mt-1">
	{#each previewSpeeches as absence}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<AbsenceBar {absence} page={0}></AbsenceBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
