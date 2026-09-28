<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import CustomBarChart from '$lib/components/Statistics/charts/CustomBarChart.svelte';
	import TwoThirdsBreakdown from '$lib/components/Statistics/TwoThirdsBreakdown.svelte';
	import { justPostStatistics } from '$lib/api/api';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import Container from '$lib/components/Layout/Container.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { getParliament } from '$lib/api/parliament';
	import type { StatisticsData } from '$lib/types';

	type OutcomeRow = { gp: string; accepted: string | null; total_initiatives: number };
	type PeriodOutcomes = {
		period: string;
		accepted: number;
		declined: number;
		preDeclined: number;
		noResult: number;
		total: number;
	};
	type View = 'stacked' | 'rate' | 'trend' | 'party' | 'topic';

	let outcomes = $state<PeriodOutcomes[]>([]);
	let selectedOutcome = $state('all');
	let selectedView = $state<View>('stacked');
	let loading = $state(true);
	let error = $state(false);
	let lineScale = $state<[number, number]>([0, 100]);
	const isEu = getParliament() === 'eu';
	let outcomeRequest: Promise<PeriodOutcomes[]> | null = null;

	const views: { value: View; label: string }[] = [
		{ value: 'stacked', label: t('statistics.twoThirds.outcomeDistribution') },
		{ value: 'rate', label: t('statistics.twoThirds.acceptanceRate') },
		{ value: 'trend', label: t('statistics.twoThirds.countTrend') },
		{ value: 'party', label: t('statistics.twoThirds.byParty') },
		{ value: 'topic', label: t('statistics.twoThirds.byTopic') }
	];
	const outcomeOptions = [
		{ value: 'all', label: t('filterOption.any') },
		{ value: 'a', label: t('filterOption.acceptedYes') },
		{ value: 'd', label: t('filterOption.acceptedNo') },
		{ value: 'p', label: t('filterOption.acceptedEarlyRejected') }
	];

	function getOutcomes(): Promise<PeriodOutcomes[]> {
		if (outcomeRequest) return outcomeRequest;
		outcomeRequest = (async () => {
			const [periods, response] = await Promise.all([
				cachedAllLegisPeriods(),
				justPostStatistics<OutcomeRow[]>('legislative_initiative_outcomes_by_period', {})
			]);
			if ('error' in response) throw new Error(t('statistics.error.load'));
			const periodNames = new Set((periods ?? []).map((period) => period.gp));
			const counts = new Map<string, PeriodOutcomes>();
			for (const period of periodNames) {
				counts.set(period, {
					period,
					accepted: 0,
					declined: 0,
					preDeclined: 0,
					noResult: 0,
					total: 0
				});
			}
			for (const row of response) {
				const current = counts.get(row.gp) ?? {
					period: row.gp,
					accepted: 0,
					declined: 0,
					preDeclined: 0,
					noResult: 0,
					total: 0
				};
				const count = Number(row.total_initiatives) || 0;
				current.total += count;
				if (row.accepted === 'a') current.accepted += count;
				else if (row.accepted === 'd') current.declined += count;
				else if (row.accepted === 'p') current.preDeclined += count;
				else current.noResult += count;
				counts.set(row.gp, current);
			}
			const romanValue = (value: string) => {
				const values: Record<string, number> = {
					I: 1,
					V: 5,
					X: 10,
					L: 50,
					C: 100,
					D: 500,
					M: 1000
				};
				let total = 0;
				for (let i = 0; i < value.length; i++) {
					const current = values[value[i]] ?? 0;
					const next = values[value[i + 1]] ?? 0;
					total += current < next ? -current : current;
				}
				return total;
			};
			return [...counts.values()].sort((a, b) => romanValue(a.period) - romanValue(b.period));
		})();
		return outcomeRequest;
	}

	onMount(async () => {
		try {
			outcomes = await getOutcomes();
		} catch {
			error = true;
		} finally {
			loading = false;
		}
	});

	let stackedSegments = $derived([
		{ key: 'accepted', label: t('filterOption.acceptedYes'), color: '#22c55e' },
		{ key: 'declined', label: t('filterOption.acceptedNo'), color: '#ef4444' },
		{ key: 'preDeclined', label: t('filterOption.acceptedEarlyRejected'), color: '#f59e0b' },
		{ key: 'noResult', label: t('statistics.twoThirds.noResult'), color: '#94a3b8' }
	]);

	function acceptanceRate(period: PeriodOutcomes) {
		const decided = period.accepted + period.declined;
		return decided > 0 ? (period.accepted / decided) * 100 : null;
	}

	const outcomeChartData = $derived(
		outcomes.map((period) => ({
			category: period.period,
			value: period.total,
			party: '',
			color: '#94a3b8',
			valueLabel: t('statistics.twoThirds.outcomeDistribution'),
			segments: {
				accepted: period.accepted,
				declined: period.declined,
				preDeclined: period.preDeclined,
				noResult: period.noResult
			}
		}))
	);
	const rateChartData = $derived(
		outcomes.map((period) => ({
			category: period.period,
			value: acceptanceRate(period) ?? 0,
			missing: acceptanceRate(period) === null,
			party: '',
			color: '#22c55e',
			valueLabel: t('statistics.twoThirds.acceptanceRate')
		}))
	);

	async function loadCountTrend(): Promise<StatisticsData[]> {
		const data = outcomes.map((period) => ({
			type: 'category' as const,
			label: period.period,
			value:
				selectedOutcome === 'all'
					? period.total
					: selectedOutcome === 'a'
						? period.accepted
						: selectedOutcome === 'd'
							? period.declined
							: period.preDeclined,
			metadata: {}
		}));
		const max = Math.max(0, ...data.map((item) => item.value));
		const step = 10 ** Math.floor(Math.log10(Math.max((max * 1.1) / 5, 1)));
		lineScale = [0, Math.max(step * 5, Math.ceil((max * 1.1) / step) * step)];
		return data;
	}
</script>

{#snippet analysisSelection(filters?: Snippet)}
	<section
		class="mb-5 rounded-xl border border-gray-300 bg-surface-50/95 p-4 shadow-sm dark:border-surface-700 dark:bg-surface-700/95"
	>
		<div class="flex flex-col gap-3 xl:flex-row xl:items-end xl:justify-between">
			<div>
				<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">
					{t('statistics.chartControl.analysis')}
				</p>
				<div class="mt-2 flex flex-wrap items-center gap-2">
					<div
						class="flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400"
					>
						{#each views as view}
							<button
								type="button"
								class="rounded-lg px-3 py-1.5 text-sm font-semibold transition {selectedView ===
								view.value
									? 'bg-primary-300 text-black dark:bg-primary-400'
									: 'hover:bg-primary-100 dark:hover:bg-surface-500'}"
								onclick={() => (selectedView = view.value)}>{view.label}</button
							>
						{/each}
					</div>
				</div>
			</div>
			{@render filters?.()}
		</div>
	</section>
{/snippet}

{#if !isEu}
	<Container class="pb-12">
		<div class="mt-2 mb-6">
			<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.twoThirds.title')}</h1>
			<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
				{t('statistics.twoThirds.intro')}
			</p>
		</div>

		{#if selectedView !== 'party' && selectedView !== 'topic' && (selectedView !== 'trend' || loading || error)}
			{@render analysisSelection()}
		{/if}

		{#if selectedView === 'party' || selectedView === 'topic'}
			<TwoThirdsBreakdown
				mode={selectedView}
				periods={outcomes.map((item) => item.period)}
				controls={analysisSelection}
			/>
		{:else if loading}
			<div
				class="flex min-h-64 items-center justify-center rounded-xl border border-gray-200 bg-surface-50 dark:border-surface-700 dark:bg-surface-800"
			>
				{t('statistics.loading')}
			</div>
		{:else if error}
			<div
				class="flex min-h-64 items-center justify-center rounded-xl border border-gray-200 bg-surface-50 text-red-600 dark:border-surface-700 dark:bg-surface-800"
			>
				{t('statistics.error.load')}
			</div>
		{:else if selectedView === 'trend'}
			<StatisticsChartControl
				controls={analysisSelection}
				height={520}
				makeRequest={loadCountTrend}
				selectedCategory="legis"
				selectedChartMode="line"
				reloadKey={selectedOutcome}
				analysisFilterLabel={t('statistics.twoThirds.outcomeFilter')}
				analysisFilterOptions={outcomeOptions}
				bind:analysisFilterValue={selectedOutcome}
				valueLabel={t('statistics.twoThirds.count')}
				normalizedValueLabel={t('statistics.twoThirds.count')}
				categoryOptions={[{ value: 'legis', label: t('statistics.legislature') }]}
				filterConfig={{
					showNormalized: false,
					showPeriod: false,
					showGender: false,
					showParty: false
				}}
				showDonutMode={false}
				lineValueDomain={lineScale}
				valuePrecision={0}
			/>
		{:else if selectedView === 'stacked'}
			<section
				class="rounded-xl border border-gray-200 bg-surface-50 shadow-sm dark:border-surface-700 dark:bg-surface-800"
			>
				<CustomBarChart
					data={outcomeChartData}
					height={520}
					metricLabel={t('statistics.twoThirds.outcomeDistribution')}
					chartDescription={t('statistics.twoThirds.outcomeDistributionDescription')}
					selectedCategory="legis"
					series={stackedSegments}
					valuePrecision={0}
					showRanks={false}
				/>
			</section>
		{:else if selectedView === 'rate'}
			<section
				class="rounded-xl border border-gray-200 bg-surface-50 shadow-sm dark:border-surface-700 dark:bg-surface-800"
			>
				<CustomBarChart
					data={rateChartData}
					height={520}
					metricLabel={t('statistics.twoThirds.acceptanceRate')}
					chartDescription={t('statistics.twoThirds.acceptanceRateDescription')}
					selectedCategory="legis"
					valueDomain={[0, 100]}
					valueSuffix="%"
					valuePrecision={1}
					showRanks={false}
				/>
			</section>
		{/if}
	</Container>
{/if}
