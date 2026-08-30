<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { NamedVote } from '$lib/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		namedVotes: NamedVote[];
	}

	let { namedVotes }: Props = $props();

	const ENTRIES = 14;

	let page = $state(1);

	let currentNamedVotes: NamedVote[] = $derived(
		namedVotes.slice((page - 1) * ENTRIES, page * ENTRIES)
	);
</script>

<div class="card px-4">
	<div class="flex items-center justify-between py-4 px-1">
		<h1 class="text-xl font-bold lg:text-2xl">{t('namedVotes.title')}</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	{#each currentNamedVotes as namedVote}
		<NamedVoteBar {namedVote} />
	{/each}

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(namedVotes.length / ENTRIES)} />
	</div>
</div>
