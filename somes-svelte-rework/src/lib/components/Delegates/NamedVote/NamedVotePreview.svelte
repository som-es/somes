<script lang="ts">
	import type { NamedVote, Speech, SpeechesWithMaxPage } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import NamedVoteBar from './NamedVoteBar.svelte';

	export let namedVotes: NamedVote[];
	export let delegateId: number;

	$: allSpeeches = {
		type: 'component',
		component: 'allNamedVotes',
		meta: { delegateId, namedVotes }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewNamedVotes = namedVotes.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<h1 class="font-bold text-2xl">Letzte Reden</h1>
	<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(allSpeeches)}>Alle anzeigen</button>
</div>
<div class="mt-5">
{#each previewNamedVotes as namedVote}
    <!-- <div class="gap-3 rounded variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
	 <NamedVoteBar {namedVote}></NamedVoteBar>
	<!-- <GovProposalExpandableBar {govProposal} /> -->
{/each}

</div>
