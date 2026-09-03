<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import Pagination from '$lib/components/Pagination.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import { errorToNull, speeches_by_delegate_per_page } from '$lib/api/api';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Delegate } from '$lib/types';
	import type { Snippet } from 'svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { aiViewEnabledStore } from '$lib/stores/stores';

	interface Props {
		title?: string;
		speeches: FullSpeech[];
		delegateId?: number;
		maxPage?: number;
		speechHeader?: Snippet<[FullSpeech]>;
		delegates?: Delegate[];
	}

	let {
		title = t('speeches.title'),
		speeches,
		delegateId,
		maxPage = 1,
		speechHeader,
		delegates = []
	}: Props = $props();

	let page = $state(1);
	let fetchedSpeeches: FullSpeech[] | null = $state(null);
	let searchValue = $state('');

	$effect(() => {
		if (delegateId === undefined) return;
		speeches_by_delegate_per_page(delegateId, page - 1).then((res) => {
			fetchedSpeeches = errorToNull(res)?.speeches ?? [];
		});
	});

	let allSpeeches = $derived(fetchedSpeeches ?? speeches);

	let delegateById = $derived(new Map(delegates.map((d) => [d.id, d])));

	function normalize(text: string): string {
		return text
			.toLowerCase()
			.normalize('NFD')
			.replace(/[̀-ͯ]/g, '');
	}

	function searchableText(speech: FullSpeech): string {
		const delegate = delegateById.get(speech.speech.delegate_id);
		const summary = aiViewEnabledStore.value ? speech.ai_summary : null;
		return normalize(
			[
				delegate?.name,
				delegate?.party,
				summary?.short_title,
				summary?.short_summary,
				summary?.summary,
				speech.speech.about,
				speech.speech.opinion
			]
				.filter(Boolean)
				.join(' ')
		);
	}

	let visibleSpeeches = $derived.by(() => {
		const terms = normalize(searchValue).split(/\s+/).filter(Boolean);
		if (terms.length === 0) return allSpeeches;
		return allSpeeches.filter((speech) => {
			const text = searchableText(speech);
			return terms.every((term) => text.includes(term));
		});
	});

	let hintAiSummary = $derived(allSpeeches.find((s) => s.ai_summary)?.ai_summary);
</script>

<div class="card p-8">
	<div class="flex items-center justify-between">
		<h1 class="text-xl font-bold lg:text-2xl">
			{#if aiViewEnabledStore.value && hintAiSummary}
				<AiSummaryHintPopup
					aiSummary={hintAiSummary}
					align="start"
					aiGenText={t('speeches.aiHint')}
				/>
			{/if}
			{title}
		</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<div class="mt-4 flex">
		<SearchBar bind:searchValue placeholder={t('speeches.searchPlaceholder')} />
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
	{:else}
		<p class="mt-6 text-center text-gray-700 dark:text-gray-300">
			{t('speeches.noSearchResults')}
		</p>
	{/each}

	{#if delegateId !== undefined && maxPage > 1}
		<div class="float-right">
			<Pagination bind:dynPage={page} {maxPage} />
		</div>
	{/if}
</div>
