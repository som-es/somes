<script lang="ts">
	import {
		errorToNull,
		interjections_made_by_delegate_per_page,
		interjections_received_by_delegate_per_page
	} from '$lib/api/api';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { HasError, Interjection, InterjectionsWithMaxPage } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import InterjectionBar from './InterjectionBar.svelte';
	import Pagination from '$lib/components/Pagination.svelte';

	interface Props {
		delegateId: number;
		interjectionsPage0: InterjectionsWithMaxPage;
		ty: 'issued' | 'received';
	}

	let { delegateId, interjectionsPage0, ty }: Props = $props();

	let currentPageInterjections = $derived(interjectionsPage0.interjections);

	const interjectionsPerPage = $derived(
		ty === 'issued'
			? interjections_made_by_delegate_per_page
			: interjections_received_by_delegate_per_page
	);

	let page = $state(1);
	$effect(() => {
		interjectionsPerPage(delegateId, page - 1).then((res) => {
			currentPageInterjections = errorToNull(res)?.interjections ?? [];
		});
	});
</script>

<div class="card px-4">
	<div class="flex items-center justify-between p-8">
		<h1 class="text-2xl font-bold">
			Alle {ty === 'issued' ? 'vergebenen' : 'erhaltenen'} Zwischenrufe
		</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<div class="flex flex-col gap-2">
		{#if currentPageInterjections.length === 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">Keine</div>
		{/if}
		{#each currentPageInterjections as interjection}
			<InterjectionBar
				{interjection}
				{ty}
				coloring="bg-primary-400 dark:bg-primary-300 text-black! "
			/>
		{/each}
	</div>

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={interjectionsPage0.max_page} />
	</div>
</div>
