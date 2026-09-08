<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
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

	let currentPageDecrees: DecreeDelegate[] = $derived(
		decrees.slice((page - 1) * ENTRIES, page * ENTRIES)
	);
</script>

<div class="w-7xl max-w-7xl card p-8">
	<div class="flex items-center justify-between">
		<h1 class="text-xl font-bold lg:text-2xl">{t('decrees.title')}</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	{#each currentPageDecrees as decree}
		<DecreeBar {decree} />
	{/each}
	{#if currentPageDecrees.length == 0}
		{#each { length: 15 } as _}
			<ExpandablePlaceholder class="w-7xl min-w-7xl" />
		{/each}
	{/if}
	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(decrees.length / ENTRIES)} />
	</div>
</div>
