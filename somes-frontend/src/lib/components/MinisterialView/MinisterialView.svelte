<script lang="ts">
	import Documents from '$lib/components/Documents/Documents.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import type { Snippet } from 'svelte';
	import AiSummaryHintPopup from '../AiHint/AiSummaryHintPopup.svelte';
	import DelegateCard from '../Delegates/DelegateCard.svelte';
	import Topics from '../Topics/Topics.svelte';
	import GlossaryText from '../UI/GlossaryText.svelte';
	import InfoBadgesCustom from '../VoteResults/InfoTiles/InfoBadgesCustom.svelte';
	import type { MinisterialViewData } from './types';

	import { dashDateToDotDate } from '$lib/date';
	import linkIcon from '$lib/assets/misc_icons/external-link.svg?raw';
	import { aiViewEnabledStore } from '$lib/stores/stores';

	interface Props {
		ministerialData: MinisterialViewData;
		children?: Snippet
	}

	let { ministerialData, children }: Props = $props();

	let aiSummary = $derived(ministerialData.aiSummary);
  let date = $derived(dashDateToDotDate(ministerialData.date.toString().split('T')[0]));
	let displayAiSummary = $derived(aiViewEnabledStore.value && aiSummary);
</script>

<title>
	{#if aiViewEnabledStore.value && ministerialData.aiSummary}
		{ministerialData.aiSummary.short_title}
	{:else}
		{ministerialData.alternativeTitle}
	{/if}
</title>


<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex max-lg:flex-wrap gap-3">
	<div class="flex flex-col gap-2 w-full">
		<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-6 py-5">
			<div class="flex justify-between items-start">
				<div class="flex items-center gap-4">
					<div class="flex flex-col">
						<div class="flex items-start gap-2">
							<span
								class="text-xl lg:text-3xl font-bold leading-tight"
								style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
							>
								{#if aiViewEnabledStore.value && aiSummary}
									<AiSummaryHintPopup aiSummary={aiSummary} />
									{aiSummary.short_title}
								{:else}
									{ministerialData.alternativeTitle}
								{/if}
							</span>
						</div>

						<span class="text-sm opacity-90">
							{ministerialData.type == "decree" ? "Verordnung" : "Ministerialentwurf"} vom {date}
						</span>
					</div>
				</div>

				<div class="flex flex-wrap items-center gap-2 flex-shrink-0">
					<a href={ministerialData.originalDocumentUrl} target="_blank" class="w-5 text-gray-500 dark:text-gray-300">
						{@html linkIcon}
					</a>
				</div>
			</div>

			{#if ministerialData.aiSummary}
				<div class="mt-5 pb-3">
					<h1 class="font-semibold text-lg md:text-xl">Zusammenfassung</h1>
					<span class="text-base lg:text-base text-gray-800 dark:text-gray-200">
						<GlossaryText
							text={ministerialData.aiSummary.short_summary}
							glossary={ministerialData.aiSummary.full_summary.glossary}
						/>
					</span>
				</div>
			{/if}

			<div class="flex flex-wrap justify-between items-center gap-3 w-full pt-1">
				<div>
					<InfoBadgesCustom texts={ministerialData.infoBadges} />
				</div>

				<div class="flex-1 flex justify-end">
					{#if aiViewEnabledStore.value && aiSummary && ministerialData.eurovocTopics.length == 0}
						<Topics topics={aiSummary.full_summary.topics.sort((a, b) => {
								return a.length - b.length;
							}).map(topic => {return {topic}})} />
					{:else}
						<Topics
							topics={ministerialData.eurovocTopics.sort((a, b) => {
								return a.topic.length - b.topic.length;
							})}
						/>
					{/if}
				</div>
			</div>
		</div>

		{#if aiViewEnabledStore.value && ministerialData.aiSummary}
			<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 pt-3 pb-3">
				<h1 class="font-bold text-lg md:text-xl">Zusammenfassung</h1>
				<span class="text-sm lg:text-base">
					<GlossaryText text={ministerialData.aiSummary.short_summary} glossary={ministerialData.aiSummary.full_summary.glossary} />
				</span>
			</div>
			<Emphasis 
				emphasis={ministerialData.aiSummary.full_summary.key_points} 
				glossary={ministerialData.aiSummary.full_summary.glossary} 
			/>
		{/if}
		<div class="flex flex-wrap gap-2 w-full">
			{#if ministerialData.documents.length > 0}
				<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
					<Documents documents={ministerialData.documents} />
				</div>
			{/if}
		</div>
		{#if children}
			{@render children()}
		{/if}
	</div>

	<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
		<DelegateCard delegate={ministerialData.delegate} />
	</div>	
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 11px;
		gap: 10px;
	}
</style>
