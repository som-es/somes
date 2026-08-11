<script lang="ts">
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Snippet } from 'svelte';
	import AllSpeechesModal from './AllSpeechesModal.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';

	interface Props {
		speeches: FullSpeech[];
		totalCount: number;
		title?: string;
		delegateId?: number;
		maxPage?: number;
		speechHeader?: Snippet<[FullSpeech]>;
	}

	let {
		speeches,
		totalCount,
		title = 'Letzte Reden',
		delegateId,
		maxPage,
		speechHeader
	}: Props = $props();

	const PREVIEW_COUNT = 2;
	let previewSpeeches = $derived(speeches.slice(0, PREVIEW_COUNT));
</script>

<div>
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0">
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">{title}</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{totalCount}
				{totalCount === 1 ? 'Rede' : 'Reden'} insgesamt
			</h2>
		</div>

		<div class="flex shrink-0 flex-col items-end gap-2">
			<ExtendInfoDialog title="Alle anzeigen">
				<AllSpeechesModal {title} {speeches} {delegateId} {maxPage} {speechHeader} />
			</ExtendInfoDialog>
			<div class="flex items-center gap-4 text-sm text-black dark:text-white">
				<div class="flex items-center gap-2">
					<span class="h-2 w-2 rounded-full bg-green-600"></span>Pro
				</div>
				<div class="flex items-center gap-2">
					<span class="h-2 w-2 rounded-full bg-red-500"></span>Contra
				</div>
			</div>
		</div>
	</div>
</div>
<div>
	{#each previewSpeeches as speech (speech.id)}
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
</div>
