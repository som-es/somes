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
	import { t } from '$lib/i18n/i18n.svelte';

	import { dashDateToDotDate } from '$lib/date';
	import linkIcon from '$lib/assets/misc_icons/external-link.svg?raw';
	import { aiViewEnabledStore, currentDelegateStore } from '$lib/stores/stores';
	import { plink } from '$lib/api/parliament';
	import { gotoHistory } from '$lib/goto';

	interface Props {
		ministerialData: MinisterialViewData;
		children?: Snippet;
		snippets?: Record<string, Snippet | undefined>;
	}

	let { ministerialData, children, snippets = {} }: Props = $props();

	let aiSummary = $derived(ministerialData.aiSummary);
	let date = $derived(dashDateToDotDate(ministerialData.date.toString().split('T')[0]));
	let displayAiSummary = $derived(aiViewEnabledStore.value && aiSummary);

	let currentDelegateIndex = $state(0);
	const delegates = $derived(ministerialData.delegates ?? []);

	function prevDelegate() {
		if (currentDelegateIndex > 0) currentDelegateIndex--;
	}

	function nextDelegate() {
		if (currentDelegateIndex < delegates.length - 1) currentDelegateIndex++;
	}

	const onShowDetails = () => {
		currentDelegateStore.value = delegates[currentDelegateIndex];
		if (delegates[currentDelegateIndex]) {
			const link = plink(
				`/delegates?gp=${ministerialData.gp}&date=${ministerialData.date.toString().split('T')[0]}&delegate=${delegates[currentDelegateIndex].id}`
			);
			gotoHistory(link, true);
		}
	};
</script>

<title>
	{#if aiViewEnabledStore.value && ministerialData.aiSummary}
		{ministerialData.aiSummary.short_title}
	{:else}
		{ministerialData.alternativeTitle}
	{/if}
</title>

<div class="mt-3 flex gap-3 max-lg:flex-wrap">
	<div class="flex w-full flex-col gap-2">
		<div class="rounded-xl bg-primary-300 px-6 py-5 dark:bg-primary-500">
			<div class="flex items-start justify-between">
				<div class="flex items-center gap-4">
					<div class="flex flex-col">
						<div class="flex items-start gap-2">
							<span
								class="text-xl leading-tight font-bold lg:text-3xl"
								style="hyphens: auto; word-break: break-word; overflow-wrap: break-word;"
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
					<h1 class="text-lg font-semibold md:text-xl">{t('ministerialView.summary')}</h1>
					<span
						class="text-base text-gray-800 lg:text-base dark:text-gray-200"
						style="hyphens: auto; word-break: break-word; overflow-wrap: break-word;"
					>
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
			<Emphasis summary={ministerialData.aiSummary.full_summary} />
		{/if}
		{#if snippets['mood']}
			{@render snippets['mood']()}
		{/if}
		{#if ministerialData.documents.length > 0 && snippets['voteable'] == null}
			<div class="flex min-w-full flex-wrap gap-2">
				<div class="min-w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
					<Documents documents={ministerialData.documents} />
				</div>
			</div>
		{/if}
		{#if snippets['voteable']}
			{@render snippets['voteable']()}
		{/if}
	</div>

	{#if delegates.length > 0}
		<div class="flex w-115 flex-col gap-2 rounded-xl bg-primary-300 px-3 py-3 dark:bg-primary-500">
			<DelegateCard
				delegate={delegates[currentDelegateIndex]}
				showMoreDetailsBtn
				{onShowDetails}
				onlyTop
				showAI={false}
				date={ministerialData.date}
			/>
			{#if delegates.length > 1}
				<div class="flex items-center justify-center gap-3">
					<button
						onclick={prevDelegate}
						disabled={currentDelegateIndex === 0}
						class="group flex gap-1 rounded-lg bg-primary-300 px-2 py-1 disabled:cursor-not-allowed disabled:opacity-40 dark:bg-primary-500"
					>
						<span class="transition-transform group-hover:-translate-x-1">&#8592;</span>
					</button>

					<span
						class="min-w-[3rem] text-center text-sm font-medium text-gray-600 dark:text-gray-300"
					>
						{currentDelegateIndex + 1}/{delegates.length}
					</span>

					<button
						onclick={nextDelegate}
						disabled={currentDelegateIndex === delegates.length - 1}
						class="group flex gap-1 rounded-lg bg-primary-300 px-2 py-1 disabled:cursor-not-allowed disabled:opacity-40 dark:bg-primary-500"
					>
						<span class="transition-transform group-hover:translate-x-1">&#8594;</span>
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>
{#if ministerialData.documents.length > 0 && ministerialData.type === 'gov_proposal' && snippets['voteable']}
	<div class="mt-2 flex min-w-full flex-wrap gap-2">
		<div class="min-w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
			<Documents documents={ministerialData.documents} />
		</div>
	</div>
{/if}

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 11px;
		gap: 10px;
	}
</style>
