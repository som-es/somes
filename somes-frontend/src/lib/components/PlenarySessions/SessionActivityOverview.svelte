<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	import type { SessionActivityOverview } from '$lib/types';

	interface ActivityCard {
		label: string;
		value: string;
		detail: string;
		highlight: boolean;
	}

	interface Props {
		overview: SessionActivityOverview | null;
	}

	let { overview }: Props = $props();

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
			return 'kein Vergleichswert';
		}

		return `Top-5%-Grenze: ${Math.round(threshold)}${unit}`;
	}

	let topSpeaker = $derived(overview?.top_speakers.at(0) ?? null);
	let activityCards: ActivityCard[] = $derived.by(() => {
		if (!overview) {
			return [];
		}

		return [
			{
				label: 'Abstimmungen',
				value: overview.vote_count.toString(),
				detail: formatPercentile(overview.percentiles.vote_count_p95),
				highlight: isTopFivePercent(overview.vote_count, overview.percentiles.vote_count_p95)
			},
			{
				label: 'Redner:innen',
				value: overview.speaker_count.toString(),
				detail: `${overview.speech_count} Redebeiträge`,
				highlight: isTopFivePercent(overview.speaker_count, overview.percentiles.speaker_count_p95)
			},
			{
				label: 'Redezeit',
				value: topSpeaker ? formatDuration(topSpeaker.total_speech_time) : formatDuration(0),
				detail: topSpeaker
					? `${topSpeaker.delegate_name} (${topSpeaker.delegate_party})`
					: 'Keine erfasste Redezeit',
				highlight: topSpeaker
					? isTopFivePercent(
							topSpeaker.total_speech_time,
							overview.percentiles.delegate_speech_time_p95
						)
					: false
			},
			{
				label: 'Komplexität',
				value: overview.average_complexity.toFixed(2),
				detail: `Schnitt der behandelten Vorlagen`,
				highlight: isTopFivePercent(
					overview.average_complexity,
					overview.percentiles.complexity_p95
				)
			},
			{
				label: 'Abwesenheiten',
				value: overview.absence_count.toString(),
				detail: formatPercentile(overview.percentiles.absence_count_p95),
				highlight: isTopFivePercent(overview.absence_count, overview.percentiles.absence_count_p95)
			},
			{
				label: 'Ordnungsrufe',
				value: overview.call_to_order_count.toString(),
				detail:
					overview.call_to_orders.length > 0
						? overview.call_to_orders
								.map((entry) => `${entry.delegate_name} (${entry.total_order_calls})`)
								.join(', ')
						: 'Keine Ordnungsrufe',
				highlight: overview.call_to_order_count > 0
			}
		];
	});
</script>

{#if overview}
	<section class="mt-10">
		<div class="mb-4 flex flex-col gap-1 px-1 sm:px-0">
			<h2 class="text-3xl font-bold sm:text-4xl">Aktivität in der letzten Sitzung</h2>
			<p class="text-base text-gray-800 dark:text-gray-200">
				{#if overview.inr}
					{overview.inr}. Nationalratssitzung
				{:else}
					Nationalratssitzung
				{/if}
				{#if overview.legislative_period}
					| {overview.legislative_period}
				{/if}
				{#if overview.date}
					| {dashDateToDotDate(overview.date)}
				{/if}
			</p>
		</div>

		<div class="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
			{#each activityCards as card}
				<article
					class="rounded-lg border p-4 shadow-sm {card.highlight
						? 'border-secondary-300 bg-secondary-50 dark:border-secondary-700 dark:bg-secondary-900/40'
						: 'border-gray-200 bg-white dark:border-surface-700 dark:bg-surface-800'}"
				>
					<div class="flex items-start justify-between gap-3">
						<div class="min-w-0">
							<p class="text-sm font-semibold text-gray-700 dark:text-gray-300">{card.label}</p>
							<p class="mt-1 text-3xl font-bold text-gray-950 dark:text-white">{card.value}</p>
						</div>
						{#if card.highlight}
							<span
								class="shrink-0 rounded-lg bg-secondary-200 px-2 py-1 text-xs font-bold text-secondary-900 dark:bg-secondary-700 dark:text-secondary-50"
								>auffällig</span
							>
						{/if}
					</div>
					<p class="mt-3 line-clamp-2 text-sm text-gray-700 dark:text-gray-300">
						{card.detail}
					</p>
				</article>
			{/each}
		</div>

		{#if overview.top_speakers.length > 1 || overview.call_to_orders.length > 1}
			<div class="mt-3 grid gap-3 lg:grid-cols-2">
				{#if overview.top_speakers.length > 1}
					<div
						class="rounded-lg border border-gray-200 bg-white p-4 shadow-sm dark:border-surface-700 dark:bg-surface-800"
					>
						<h3 class="text-base font-bold">Längste Redezeiten</h3>
						<div class="mt-3 space-y-2">
							{#each overview.top_speakers as speaker}
								<div class="flex items-center justify-between gap-3 text-sm">
									<span class="min-w-0 truncate">
										{speaker.delegate_name}
										<span class="text-gray-600 dark:text-gray-400">({speaker.delegate_party})</span>
									</span>
									<span class="shrink-0 font-semibold">
										{formatDuration(speaker.total_speech_time)}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				{#if overview.call_to_orders.length > 1}
					<div
						class="rounded-lg border border-gray-200 bg-white p-4 shadow-sm dark:border-surface-700 dark:bg-surface-800"
					>
						<h3 class="text-base font-bold">Ordnungsrufe</h3>
						<div class="mt-3 space-y-2">
							{#each overview.call_to_orders as callToOrder}
								<div class="flex items-center justify-between gap-3 text-sm">
									<span class="min-w-0 truncate">
										{callToOrder.delegate_name}
										<span class="text-gray-600 dark:text-gray-400"
											>({callToOrder.delegate_party})</span
										>
									</span>
									<span class="shrink-0 font-semibold">
										{callToOrder.total_order_calls}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</section>
{/if}
