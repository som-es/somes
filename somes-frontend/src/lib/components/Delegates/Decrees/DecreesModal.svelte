<script lang="ts">
	import Pagination from '$lib/components/Pagination.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { Dialog } from 'bits-ui';
	import DecreeBar from './DecreeBar.svelte';
	import type { Decree, DecreeDelegate } from './types';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		decrees: DecreeDelegate[];
	}

	let { decrees }: Props = $props();

	const ENTRIES = 15;
	let page = $state(1);

	let currentPageDecrees: DecreeDelegate[] = $derived(decrees.slice((page - 1) * ENTRIES, page * ENTRIES));

</script>

<div class="card p-8 max-w-7xl w-7xl">
	<div class="flex justify-between items-center">
		<h1 class="font-bold text-2xl">Letzte Verordnungen</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>	
	</div>

	{#each currentPageDecrees as decree}
		<DecreeBar {decree} />
	{/each}
	{#if currentPageDecrees.length == 0}
		{#each { length: 15 } as _}
			<ExpandablePlaceholder class="min-w-7xl w-7xl" />
		{/each}
	{/if}
	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(decrees.length / ENTRIES)} />
	</div>
</div>
