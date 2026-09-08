<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import type { PlenarySession, SessionActivityOverview } from '$lib/types';
	import { defaultGpByParliament, type Parliament } from '$lib/api/parliament';

	interface ActivityCard {
		label: string;
		value: string;
		detail: string;
		highlight: boolean;
		className: string;
	}

	interface Props {
		overview: SessionActivityOverview | null;
		plenarySessions: Record<string, PlenarySession[]> | null;
		parliament: Parliament;
	}

	let { overview, plenarySessions, parliament }: Props = $props();

	function isTopFivePercent(value: number, threshold: number): boolean {
		return threshold > 0 && value >= threshold;
	}

	function formatDuration(seconds: number): string {
		const safeSeconds = Math.max(0, seconds);
		const hours = Math.floor(safeSeconds / 3600);
		const minutes = Math.round((safeSeconds % 3600) / 60);

		if (hours > 0) {
			return `${hours} h ${minutes} min`;
		}

		return `${minutes} min`;
	}

	function formatPercentile(threshold: number, unit = ''): string {
		if (threshold <= 0) {
			return t('sessionActivity.percentile.noComparison');
		}

		return t('sessionActivity.percentile.top5', { value: Math.round(threshold) }) + unit;
	}

	let averageSpeechTime = $derived(
		overview && overview.speech_count > 0 ? overview.total_speech_time / overview.speech_count : 0
	);
	let activityCards: ActivityCard[] = $derived.by(() => {
		if (!overview) {
			return [];
		}

		return [
			{
				label: t('common.votes'),
				value: overview.vote_count.toString(),
				detail: formatPercentile(overview.percentiles.vote_count_p95),
				highlight: isTopFivePercent(overview.vote_count, overview.percentiles.vote_count_p95),
				className: 'xl:col-span-4'
			},
			{
				label: t('sessionActivity.absences'),
				value: overview.absence_count.toString(),
				detail: formatPercentile(overview.percentiles.absence_count_p95),
				highlight: isTopFivePercent(overview.absence_count, overview.percentiles.absence_count_p95),
				className: 'xl:col-span-4'
			},
			{
				label: t('sessionActivity.avgSpeechTime'),
				value: formatDuration(averageSpeechTime),
				detail: t('sessionActivity.detail.speechCountTotal', {
					count: overview.speech_count,
					total: formatDuration(overview.total_speech_time)
				}),
				highlight: false,
				className: 'col-span-2 xl:col-span-4'
			},
			{
				label: t('sessionActivity.speakers'),
				value: overview.speaker_count.toString(),
				detail: t('sessionActivity.detail.speechCount', { count: overview.speech_count }),
				highlight: isTopFivePercent(overview.speaker_count, overview.percentiles.speaker_count_p95),
				className: 'xl:col-span-2'
			},
			{
				label: t('sessionActivity.orderCalls'),
				value: overview.call_to_order_count.toString(),
				detail: t('sessionActivity.noOrderCalls'),
				items: overview.call_to_orders.map(
					(entry) => `${entry.delegate_name} (${entry.total_order_calls})`
				),
				highlight: overview.call_to_order_count > 0,
				className: 'xl:col-span-3'
			}
		];
	});
</script>

{#if overview}
	<section class="mt-10">
		<div class="mb-4 flex flex-col gap-1 px-1 sm:px-0">
			<h2 class="text-2xl font-bold sm:text-4xl">{t('sessionActivity.lastSessionTitle')}</h2>
			<p class="text-base text-gray-800 dark:text-gray-200">
				{#if parliament === 'eu' && plenarySessions}
					{plenarySessions[defaultGpByParliament(parliament)].length}. {t(
						'sessionActivity.sessionLabelEu'
					)}
				{:else}
					{#if overview.inr}{overview.inr}.{/if}
					{t('sessionActivity.sessionLabel')}
				{/if}

				{#if overview.legislative_period}
					| {overview.legislative_period}
				{/if}
				{#if overview.date}
					| {formatDate(new Date(overview.date))}
				{/if}
			</p>
		</div>

		<!-- Mobile: 2 columns, desktop (xl): 12 columns – each card sets its own col-span -->
		<div class="grid grid-cols-2 gap-3 xl:grid-cols-12">
			{#each activityCards as card}
				<article
					class="rounded-xl bg-primary-300 p-3 shadow-sm sm:p-4 dark:bg-primary-500 {card.className}"
				>
					<div class="flex items-start justify-between gap-3">
						<div class="min-w-0">
							<p class="text-sm font-semibold text-gray-800 dark:text-gray-100">{card.label}</p>
							<p class="mt-1 text-2xl font-bold text-gray-950 sm:text-3xl dark:text-white">
								{card.value}
							</p>
						</div>
						{#if card.highlight}
							<span
								class="shrink-0 rounded-lg bg-secondary-500 px-2 py-1 text-xs font-bold text-white dark:bg-secondary-300 dark:text-gray-950"
							>
								{t('sessionActivity.highlight')}
							</span>
						{/if}
					</div>
					{#if card.items && card.items.length > 0}
						<ul class="mt-3 flex flex-col gap-1 text-sm text-gray-700 dark:text-gray-100">
							{#each card.items as item}
								<li>{item}</li>
							{/each}
						</ul>
					{:else}
						<p class="mt-3 text-sm text-gray-700 dark:text-gray-100">{card.detail}</p>
					{/if}
				</article>
			{/each}

			<article
				class="col-span-2 rounded-xl bg-primary-300 p-3 shadow-sm sm:p-4 xl:col-span-7 dark:bg-primary-500"
			>
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-100">
					{t('sessionActivity.topSpeakersTitle')}
				</h3>
				{#if overview.top_speakers.length > 0}
					<div class="mt-3 grid gap-2">
						{#each overview.top_speakers as speaker}
							<div class="flex items-start justify-between gap-3 text-sm">
								<div class="min-w-0 sm:hidden">
									<p>{speaker.delegate_name}</p>
									<p class="text-xs text-gray-600 dark:text-gray-400">{speaker.delegate_party}</p>
								</div>
								<div class="min-w-0 max-sm:hidden">
									<p>
										{speaker.delegate_name}
										<span class="ml-1 text-xs text-gray-600 dark:text-gray-400"
											>({speaker.delegate_party})</span
										>
									</p>
								</div>
								<span class="shrink-0 font-semibold">
									{formatDuration(speaker.total_speech_time)}
								</span>
							</div>
						{/each}
					</div>
				{:else}
					<p class="mt-3 text-sm text-gray-700 dark:text-gray-100">
						{t('sessionActivity.noSpeechTime')}
					</p>
				{/if}
			</article>
		</div>
	</section>
{/if}
