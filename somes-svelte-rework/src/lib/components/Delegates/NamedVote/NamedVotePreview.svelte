<script lang="ts">
	import type { NamedVote, Speech, SpeechesWithMaxPage } from '$lib/types';
	import { type ModalSettings } from '@skeletonlabs/skeleton-svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';

	export let namedVotes: NamedVote[];
	export let delegateId: number;

	$: allNamedVotes = {
		type: 'component',
		component: 'allNamedVotes',
		meta: { delegateId, namedVotes }
	} as ModalSettings;

	const modalStore = getModalStore();

	$: previewNamedVotes = namedVotes.slice(0, 2);
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte namentliche Abstimmungen</h1>
		<h2 class="sm:text-lg">
			{namedVotes.length}
			{namedVotes.length == 1 ? 'Abstimmung' : 'Abstimmungen'} insgesamt
		</h2>
	</div>
	<button class="btn sm:btn-lg preset-filled mt-1" on:click={() => modalStore.trigger(allNamedVotes)}
		>Alle anzeigen</button
	>
</div>
<div class="mt-5">
	{#each previewNamedVotes as namedVote}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<NamedVoteBar {namedVote}></NamedVoteBar>
	{/each}
</div>
