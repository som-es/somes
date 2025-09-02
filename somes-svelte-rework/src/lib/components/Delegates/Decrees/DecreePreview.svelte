<script lang="ts">
	import type { Absence, Decree, Speech, SpeechesWithMaxPage } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import DecreeBar from './DecreeBar.svelte';

	export let decrees: Decree[];
	export let delegateId: number;

	$: allDecrees = {
		type: 'component',
		component: 'allDecrees',
		meta: { delegateId: delegateId, decrees }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewDecrees = decrees.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte Verordnungen</h1>

		<h2 class="sm:text-lg">
			{decrees.length}
			{decrees.length == 1 ? 'Verordnung' : 'Verordnungen'} insgesamt
		</h2>
	</div>
	<button class="btn sm:btn-lg variant-filled mt-1" on:click={() => modalStore.trigger(allDecrees)}
		>Alle anzeigen</button
	>
</div>

<div class="mt-1">
	{#each previewDecrees as decree}
		<!-- <div class="gap-3 rounded variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<DecreeBar {decree} page={0}></DecreeBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
