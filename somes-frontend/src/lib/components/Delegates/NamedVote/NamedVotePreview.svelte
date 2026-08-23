<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { NamedVote, SpeechesWithMaxPage } from '$lib/types';
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
				{t('namedVotes.title')}
			</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{namedVotes.length}
				{namedVotes.length == 1 ? t('namedVotes.vote') : t('namedVotes.votes')} {t('namedVotes.total')}
			</h2>
		</div>
		<ExtendInfoDialog title={t('namedVotes.showAll')}>
			<NamedVoteModal {namedVotes} />
		</ExtendInfoDialog>
	</div>
</div>
<div>
	{#each previewNamedVotes as namedVote}
		<NamedVoteBar {namedVote}></NamedVoteBar>
	{/each}
</div>
