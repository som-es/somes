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
		children?: Snippet;
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

<div class="entry mt-3 flex gap-3 bg-primary-200 max-lg:flex-wrap dark:bg-primary-400">
	<div class="flex w-full flex-col gap-2">
		<div class="rounded-xl bg-primary-300 px-6 py-5 dark:bg-primary-500">
			<div class="flex items-start justify-between">
				<div class="flex items-center gap-4">
					<div class="flex flex-col">
						<div class="flex items-start gap-2">
							<span
								class="text-xl leading-tight font-bold lg:text-3xl"
								style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
							>
								{#if aiViewEnabledStore.value && aiSummary}
									<AiSummaryHintPopup {aiSummary} />
									{aiSummary.short_title}
								{:else}
									{ministerialData.alternativeTitle}
								{/if}
							</span>
						</div>

						<span class="text-sm opacity-90">
							{ministerialData.type == 'decree' ? 'Verordnung' : 'Ministerialentwurf'} vom {date}
						</span>
					</div>
				</div>

				<div class="flex flex-shrink-0 flex-wrap items-center gap-2">
					<a
						href={ministerialData.originalDocumentUrl}
						target="_blank"
						class="w-5 text-gray-500 dark:text-gray-300"
					>
						{@html linkIcon}
					</a>
				</div>
			</div>

			{#if ministerialData.aiSummary}
				<div class="mt-5 pb-3">
					<h1 class="text-lg font-semibold md:text-xl">Zusammenfassung</h1>
					<span class="text-base text-gray-800 lg:text-base dark:text-gray-200">
						<GlossaryText
							text={ministerialData.aiSummary.short_summary}
							glossary={ministerialData.aiSummary.full_summary.glossary}
						/>
					</span>
				</div>
			{/if}

			<div class="flex w-full flex-wrap items-center justify-between gap-3 pt-1">
				<div>
					<InfoBadgesCustom texts={ministerialData.infoBadges} />
				</div>

				<div class="flex flex-1 justify-end">
					{#if aiViewEnabledStore.value && aiSummary && ministerialData.eurovocTopics.length == 0}
						<Topics
							topics={aiSummary.full_summary.topics
								.sort((a, b) => {
									return a.length - b.length;
								})
								.map((topic) => {
									return { topic };
								})}
						/>
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
			<Emphasis
				emphasis={ministerialData.aiSummary.full_summary.key_points}
				glossary={ministerialData.aiSummary.full_summary.glossary}
			/>
		{/if}
		<div class="flex w-full flex-wrap gap-2">
			{#if ministerialData.documents.length > 0}
				<div class="rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
					<Documents documents={ministerialData.documents} />
				</div>
			{/if}
		</div>
		{#if children}
			{@render children()}
		{/if}
	</div>

	<div class="rounded-xl bg-primary-300 px-3 py-3 dark:bg-primary-500">
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
