<script lang="ts">
	import type { NamedVote, Speech, SpeechesWithMaxPage } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';
	import NamedVoteModal from './NamedVoteModal.svelte';

	interface Props {
		namedVotes: NamedVote[];
	}

	let { namedVotes }: Props = $props();

	let previewNamedVotes = $derived(namedVotes.slice(0, 2));
</script>

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">
				Letzte namentliche Abstimmungen
			</h1>
			<h2 class="text-sm text-primary-600 dark:text-primary-300">
				{namedVotes.length}
				{namedVotes.length == 1 ? 'Abstimmung' : 'Abstimmungen'} insgesamt
			</h2>
		</div>
		<ExtendInfoDialog title="Alle anzeigen">
			<NamedVoteModal {namedVotes} />
		</ExtendInfoDialog>
	</div>
</div>
<div>
	{#each previewNamedVotes as namedVote}
		<NamedVoteBar {namedVote}></NamedVoteBar>
	{/each}
</div>
