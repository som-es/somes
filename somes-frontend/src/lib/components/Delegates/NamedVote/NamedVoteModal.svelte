<script lang="ts">
	import type { NamedVote } from '$lib/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		namedVotes: NamedVote[];
		delegateId: number;
	}

	let { namedVotes, delegateId }: Props = $props();

	const ENTRIES = 14;
	
	let page = $state(1);

	let currentNamedVotes: NamedVote[] = $derived(namedVotes.slice((page - 1) * ENTRIES, page * ENTRIES));
	
</script>
<div class="card p-8 ">
	<div class="flex justify-between">

		<h1 class="font-bold text-2xl">Letzte namentliche Abstimmungen</h1>
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
