<script lang="ts">
	import { Popover } from 'bits-ui';
	import type { AiSummary, EnforcementDates, Keypoint } from '$lib/ai_summary_types';
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Delegate } from '$lib/types';
	import { localeStore } from '$lib/i18n/i18n.svelte';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import KeypointSpeakers, { type KeypointSpeaker } from './KeypointSpeakers.svelte';
	import { slide } from 'svelte/transition';
	import { t } from '$lib/i18n/i18n.svelte';

	interface Props {
		summary: AiSummary | null;
		speeches?: FullSpeech[];
		delegates?: Delegate[];
		legisInitId?: number;
	}

	let { summary, speeches = [], delegates = [], legisInitId }: Props = $props();

	const VISIBLE_COUNT = 4;

	let open = $state(false);
	let emphasis: Keypoint[] | null = $derived(summary?.key_points ?? null);
	let glossary = $derived(summary?.glossary ?? null);
	let firstPoints: Keypoint[] = $derived((emphasis ?? []).slice(0, VISIBLE_COUNT));
	let restPoints: Keypoint[] = $derived((emphasis ?? []).slice(VISIBLE_COUNT));

	let headerDates: EnforcementDates | null = $derived(
		summary?.general_enforcement_start_end ?? null
	);

	interface EnforcementEntry {
		label: string;
		date: string;
		note: string | null;
	}

	function formatEnforcementDate(value: string | null | undefined): string | null {
		if (!value) return null;

		const date = new Date(value);
		if (Number.isNaN(date.getTime())) return null;
		const locale = localeStore.value === 'de' ? 'de-AT' : 'en-AT';
		return new Intl.DateTimeFormat(locale, {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		}).format(date);
	}

	function enforcementEntries(dates: EnforcementDates): EnforcementEntry[] {
		const start = formatEnforcementDate(dates.enforcement_start_date);
		const end = formatEnforcementDate(dates.enforcement_end_date);
		const entries: EnforcementEntry[] = [];

		if (start)
			entries.push({
				label: t('emphasis.inForceFrom'),
				date: start,
				note: dates.start_notes ?? null
			});
		if (end)
			entries.push({
				label: t('emphasis.outOfForceFrom'),
				date: end,
				note: dates.end_notes ?? null
			});

		return entries;
	}

	// Was schon oben steht, muss nicht nochmal an jedem Punkt hängen.
	function sameAsHeader(dates: EnforcementDates): boolean {
		return JSON.stringify(dates) === JSON.stringify(headerDates);
	}

	let speakersByPoint = $derived.by(() => {
		const map = new Map<number, KeypointSpeaker[]>();
		if (legisInitId == null) return map;

		for (const speech of speeches) {
			const delegate = delegates.find((d) => d.id === speech.delegate_id);
			if (!delegate) continue;

			const pointsOfSpeech = new Map<number, number[]>();
			for (const relation of speech.relations ?? []) {
				if (relation.legis_init_id !== legisInitId) continue;
				for (const rel of relation.full_speech_relations.propsal_keypoint_relations ?? []) {
					for (const pointId of rel.referenced_proposal_key_point_ids) {
						const indexes = pointsOfSpeech.get(pointId) ?? [];
						if (!indexes.includes(rel.speech_key_point)) indexes.push(rel.speech_key_point);
						pointsOfSpeech.set(pointId, indexes);
					}
				}
			}

			for (const [pointId, pointIndexes] of pointsOfSpeech) {
				map.set(pointId, [...(map.get(pointId) ?? []), { delegate, speech, pointIndexes }]);
			}
		}

		return map;
	});
</script>

{#snippet clock()}
	<span
		aria-hidden="true"
		class="h-4 w-4 shrink-0 [&_path]:stroke-current [&>svg]:h-full [&>svg]:w-full"
	>
		{@html clockIcon}
	</span>
{/snippet}

{#snippet entryText(entry: EnforcementEntry)}
	{entry.label}
	<span class="font-semibold">{entry.date}</span>
	{#if entry.note}
		<span>({entry.note})</span>
	{/if}
{/snippet}

<!-- from to, for all points of the text -->
{#snippet enforcementRow(dates: EnforcementDates)}
	{@const entries = enforcementEntries(dates)}
	{#if entries.length > 0}
		<div
			class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-sm text-gray-700 dark:text-gray-300"
		>
			{@render clock()}
			{#each entries as entry, i (entry.label)}
				<span>
					{#if i > 0}
						<span aria-hidden="true" class="mr-1 text-gray-500 dark:text-gray-400">·</span>
					{/if}
					{@render entryText(entry)}
				</span>
			{/each}
		</div>
	{/if}
{/snippet}

<!-- from to, for ONE point of the text -->
{#snippet enforcementClock(dates: EnforcementDates)}
	{@const entries = enforcementEntries(dates)}
	{#if entries.length > 0}
		<Popover.Root>
			<Popover.Trigger
				openOnHover
				openDelay={0}
				title={t('emphasis.tooltipTitle')}
				class="ml-1 inline-flex translate-y-[3px] cursor-pointer text-gray-700 hover:scale-110 dark:text-gray-300"
			>
				{@render clock()}
			</Popover.Trigger>
			<Popover.Content
				align="start"
				collisionPadding={8}
				class="z-50! w-72 max-w-[calc(100vw-2rem)] card bg-primary-300-700 p-3 shadow-xl"
			>
				<div class="font-semibold">{t('emphasis.effectivePeriod')}</div>
				<ul class="mt-1 flex flex-col gap-1">
					{#each entries as entry (entry.label)}
						<li class="text-sm text-gray-800 dark:text-gray-200">{@render entryText(entry)}</li>
					{/each}
				</ul>
			</Popover.Content>
		</Popover.Root>
	{/if}
{/snippet}

{#snippet pointContent(emph: Keypoint, index: number)}
	<span class="flex gap-2 md:min-w-0 md:flex-1">
		<span class="mt-2 h-2 w-2 shrink-0 rounded-full bg-primary-500 dark:bg-primary-300"></span>
		<span class="min-w-0 flex-1">
			{#if glossary}
				<span class="text-base text-gray-800 lg:text-base dark:text-gray-200">
					<GlossaryText text={emph.point} {glossary} />
				</span>
			{:else}
				<span>{emph.point}</span>
			{/if}
			{#if emph.enforcement_start_end && !sameAsHeader(emph.enforcement_start_end)}
				{@render enforcementClock(emph.enforcement_start_end)}
			{/if}
		</span>
	</span>
	<KeypointSpeakers speakers={speakersByPoint.get(index) ?? []} pointText={emph.point} />
{/snippet}

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 px-5 pt-3 pb-3 dark:bg-primary-500">
			<div class="flex justify-between">
				<h1 class="text-lg font-semibold md:text-xl">{t('emphasis.title')}</h1>
			</div>

			{#if headerDates}
				{@render enforcementRow(headerDates)}
			{/if}

			<ul class="list mt-2 fill-primary-400 px-3">
				{#each firstPoints as emph, i}
					<li class="mb-3 md:flex md:items-start md:justify-between md:gap-3">
						{@render pointContent(emph, i)}
					</li>
				{/each}

				{#if open}
					<div transition:slide={{ duration: 240 }}>
						{#each restPoints as emph, i}
							<li class="my-3 md:flex md:items-start md:justify-between md:gap-3">
								{@render pointContent(emph, i + VISIBLE_COUNT)}
							</li>
						{/each}
					</div>
				{/if}

				{#if restPoints.length > 0}
					<button class="text-md font-semibold" onclick={() => (open = !open)}>
						<span>{open ? t('emphasis.less') : t('emphasis.more')} anzeigen</span>
					</button>
				{/if}
			</ul>
		</div>
	{:else}
		<div class="emphasis-item"></div>
	{/if}
{/if}

<style>
	.emphasis-item {
		grid-area: e;
	}
</style>
