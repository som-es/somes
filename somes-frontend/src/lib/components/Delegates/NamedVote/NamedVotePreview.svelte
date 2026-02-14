<script lang="ts">
	import type { NamedVote, Speech, SpeechesWithMaxPage } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';
	import NamedVoteModal from './NamedVoteModal.svelte';

	interface Props {
		namedVotes: NamedVote[];
		delegateId: number;
	}

	let { namedVotes, delegateId }: Props = $props();

	let previewNamedVotes = $derived(namedVotes.slice(0, 2));
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Letzte namentliche Abstimmungen</h1>
		<h2 class="sm:text-lg">
			{namedVotes.length}
			{namedVotes.length == 1 ? 'Abstimmung' : 'Abstimmungen'} insgesamt
		</h2>
	</div>
	<ExtendInfoDialog title="Alle anzeigen">
		<NamedVoteModal {delegateId} {namedVotes} />
	</ExtendInfoDialog>
</div>
<div class="mt-5">
	{#each previewNamedVotes as namedVote}
		<NamedVoteBar {namedVote}></NamedVoteBar>
	{/each}
</div>
