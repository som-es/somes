<script lang="ts">
	import DecreeBar from './DecreeBar.svelte';
	import type { Decree, DecreeDelegate } from './types';
	import type { Delegate } from '$lib/types';

	interface Props {
		decrees: Decree[];
		delegate: Delegate;
	}

	let { decrees, delegate }: Props = $props();

	// $: allDecrees = {
	// 	type: 'component',
	// 	component: 'allDecrees',
	// 	meta: { delegateId: delegate.id, decrees }
	// } as ModalSettings;

	// const modalStore = getModalStore();

	let delegateDecrees: DecreeDelegate[] = $derived(decrees.map(decree => {return {decree, delegate }}));


	let previewDecrees = $derived(delegateDecrees.slice(0, 2));
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte Verordnungen</h1>

		<h2 class="sm:text-lg">
			{decrees.length}
			{decrees.length == 1 ? 'Verordnung' : 'Verordnungen'} insgesamt
		</h2>
	</div>
	<button class="btn sm:btn-lg preset-filled mt-1" onclick={() => {}}
		>Alle anzeigen
	</button>
</div>

<div class="mt-1">
	{#each previewDecrees as decree}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<DecreeBar {decree}></DecreeBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
