<script lang="ts">
	import Pagination from '$lib/components/Pagination.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import { errorToNull, speeches_by_delegate_per_page } from '$lib/api/api';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Snippet } from 'svelte';

	interface Props {
		title?: string;
		speeches: FullSpeech[];
		delegateId?: number;
		maxPage?: number;
		speechHeader?: Snippet<[FullSpeech]>;
	}

	let { title = 'Letzte Reden', speeches, delegateId, maxPage = 1, speechHeader }: Props = $props();

	let page = $state(1);
	let fetchedSpeeches: FullSpeech[] | null = $state(null);

	$effect(() => {
		if (delegateId === undefined) return;
		speeches_by_delegate_per_page(delegateId, page - 1).then((res) => {
			fetchedSpeeches = errorToNull(res)?.speeches ?? [];
		});
	});

	let visibleSpeeches = $derived(fetchedSpeeches ?? speeches);
</script>

<div class="card p-8">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold">{title}</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	{#each visibleSpeeches as speech (speech.id)}
		{#if speechHeader}
			<SpeechBar {speech}>
				{#snippet header()}
					{@render speechHeader(speech)}
				{/snippet}
			</SpeechBar>
		{:else}
			<SpeechBar {speech} />
		{/if}
	{/each}

	{#if delegateId !== undefined && maxPage > 1}
		<div class="float-right">
			<Pagination bind:dynPage={page} {maxPage} />
		</div>
	{/if}
</div>
