<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Delegate } from '$lib/types';
	import type { Snippet } from 'svelte';
	import AllSpeechesModal from './AllSpeechesModal.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { aiViewEnabledStore } from '$lib/stores/stores';

	interface Props {
		speeches: FullSpeech[];
		totalCount: number;
		title?: string;
		delegateId?: number;
		maxPage?: number;
		speechHeader?: Snippet<[FullSpeech]>;
		delegates?: Delegate[];
	}

	let {
		speeches,
		totalCount,
		title = t('speeches.title'),
		delegateId,
		maxPage,
		speechHeader,
		delegates
	}: Props = $props();

	const PREVIEW_COUNT = 2;
	let previewSpeeches = $derived(speeches.slice(0, PREVIEW_COUNT));

	let hintAiSummary = $derived(speeches.find((s) => s.ai_summary)?.ai_summary);
</script>

<div>
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0">
			<div class="flex items-start gap-2">
				<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">
					{#if aiViewEnabledStore.value && hintAiSummary}
						<AiSummaryHintPopup
							aiSummary={hintAiSummary}
							align="start"
							aiGenText={t('speeches.aiHint')}
						/>
					{/if}
					{title}
				</h1>
			</div>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{totalCount}
				{totalCount === 1 ? t('speeches.speech') : t('speeches.speeches')}
				{t('speeches.total')}
			</h2>
		</div>

		<div class="flex shrink-0 flex-col items-end gap-2">
			<ExtendInfoDialog title={t('speeches.showAll')}>
				<AllSpeechesModal {title} {speeches} {delegateId} {maxPage} {speechHeader} {delegates} />
			</ExtendInfoDialog>
			<div class="flex items-center gap-4 text-sm text-black dark:text-white">
				<div class="flex items-center gap-2">
					<span class="h-2 w-2 rounded-full bg-green-600"></span>{t('speeches.pro')}
				</div>
				<div class="flex items-center gap-2">
					<span class="h-2 w-2 rounded-full bg-red-500"></span>{t('speeches.contra')}
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
