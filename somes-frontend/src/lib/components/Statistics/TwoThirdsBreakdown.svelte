<script lang="ts">
	import CustomBarChart from './charts/CustomBarChart.svelte';
	import type { Snippet } from 'svelte';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { createFilterGroup } from '$lib/components/Filtering/filterGroup.svelte';
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import { localeStore } from '$lib/i18n';

	let {
		mode,
		periods,
		controls
	}: {
		mode: 'party' | 'topic';
		periods: string[];
		controls?: Snippet<[Snippet]>;
	} = $props();
	type Row = {
		category: string;
		positive: number;
		negative: number;
		other: number;
		unknown: number;
		total: number;
	};
	let rows = $state<Row[]>([]);
	let searchValue = $state('');
	const periodFilter = createFilterGroup<string>({
		title: () => t('statistics.legislature'),
		hidden: () => false,
		initialValue: 'all',
		options: () => [
			{ title: t('statistics.all'), value: 'all' },
			...periods
				.slice()
				.reverse()
				.map((gp) => ({ title: gp, value: gp }))
		]
	});
	let periodInitialized = $state(false);
	$effect(() => {
		// The page supplies periods in chronological order, possibly after loading.
		if (!periodInitialized && periods.length > 0) {
			periodFilter.activeValue = periods[periods.length - 1];
			periodInitialized = true;
		}
	});

	const sortingFilter = createFilterGroup<string>({
		title: () => t('statistics.sorting'),
		hidden: () => false,
		initialValue: 'desc',
		options: () => [
			{ title: t('statistics.descending'), value: 'desc' },
			{ title: t('statistics.ascending'), value: 'asc' }
		]
	});
	const normalizationFilter = createFilterGroup<string>({
		title: () => t('statistics.normalization'),
		hidden: () => false,
		initialValue: 'normalized',
		options: () => [
			{ title: t('statistics.normalized'), value: 'normalized' },
			{ title: t('statistics.absolute'), value: 'absolute' }
		]
	});
	let partyFilters = $state([sortingFilter, normalizationFilter]);
	let topicFilters = $state([sortingFilter]);
	let loading = $state(true);
	let error = $state(false);
	let retry = $state(0);
	const numberFormat = $derived(
		new Intl.NumberFormat(localeStore.value, { maximumFractionDigits: 1 })
	);
	const isParty = $derived(mode === 'party');
	const title = $derived(
		t(isParty ? 'statistics.twoThirds.byParty' : 'statistics.twoThirds.byTopic')
	);
	const segments = $derived([
		{
			key: 'positive' as const,
			label: t(isParty ? 'statistics.twoThirds.inFavor' : 'filterOption.acceptedYes'),
			color: '#22c55e'
		},
		{
			key: 'negative' as const,
			label: t(isParty ? 'statistics.twoThirds.against' : 'filterOption.acceptedNo'),
			color: '#ef4444'
		},
		{
			key: 'other' as const,
			label: t(isParty ? 'statistics.twoThirds.abstention' : 'filterOption.acceptedEarlyRejected'),
			color: '#f59e0b'
		},
		{
			key: 'unknown' as const,
			label: t(isParty ? 'statistics.twoThirds.unclear' : 'statistics.twoThirds.noResult'),
			color: '#94a3b8'
		}
	]);
	const showPercent = $derived(isParty && normalizationFilter.activeValue !== 'absolute');
	const filteredRows = $derived.by(() => {
		const query = searchValue.trim().toLocaleLowerCase();
		const value = (row: Row) =>
			isParty ? (showPercent ? row.positive / row.total : row.positive) : row.total;
		return rows
			.filter((row) => row.category.toLocaleLowerCase().includes(query))
			.sort(
				(a, b) =>
					(sortingFilter.activeValue === 'asc' ? value(a) - value(b) : value(b) - value(a)) ||
					a.category.localeCompare(b.category)
			);
	});
	const chartRows = $derived(
		filteredRows.map((row) => ({
			category: row.category,
			value: isParty ? (showPercent ? (row.positive / row.total) * 100 : row.positive) : row.total,
			party: '',
			color: '#94a3b8',
			valueLabel: title,
			detailLabel: `${t('statistics.twoThirds.number')}: ${numberFormat.format(row.total)}`,
			segments: Object.fromEntries(
				segments.map((segment) => [
					segment.key,
					showPercent ? (row[segment.key] / row.total) * 100 : row[segment.key]
				])
			)
		}))
	);

	$effect(() => {
		const requestedMode = mode;
		const requestedPeriod = periodFilter.activeValue === 'all' ? null : periodFilter.activeValue;
		retry;
		let cancelled = false;
		loading = true;
		error = false;
		(async () => {
			try {
				const response = await justPostStatistics<Row[]>(`two_thirds_by_${requestedMode}`, {
					legis_period: requestedPeriod || null
				});
				if ('error' in response) throw new Error('Failed to load breakdown');
				if (!cancelled) rows = response;
			} catch {
				if (!cancelled) error = true;
			} finally {
				if (!cancelled) loading = false;
			}
		})();
		return () => {
			cancelled = true;
		};
	});
</script>

{#snippet filters()}
	<div class="flex min-w-0 flex-col gap-2 md:flex-row md:items-end">
		<div class="min-w-0 flex-1 md:min-w-64">
			<p class="mb-2 text-sm font-semibold text-gray-600 dark:text-gray-300">
				{t('statistics.chartControl.search')}
			</p>
			<SearchBar
				bind:searchValue
				placeholder={t('statistics.searchCategory')}
				aria-label={t('statistics.chartControl.search')}
			/>
		</div>
		<div class="flex h-10 shrink-0 gap-2 text-sm">
			{#if isParty}
				<GenericFilters bind:genericFilters={partyFilters} legisPeriodFilter={periodFilter} />
			{:else}
				<GenericFilters bind:genericFilters={topicFilters} legisPeriodFilter={periodFilter} />
			{/if}
		</div>
	</div>
{/snippet}

{#if controls}
	{@render controls(filters)}
{:else}
	<div class="mb-5">{@render filters()}</div>
{/if}

<section
	class="rounded-xl border border-gray-200 bg-surface-50 shadow-sm dark:border-surface-700 dark:bg-surface-800"
>
	{#if loading}
		<p class="p-8 text-center" role="status">{t('statistics.loading')}</p>
	{:else if error}
		<div class="p-8 text-center" role="alert">
			<p>{t('statistics.error.load')}</p>
			<button
				type="button"
				class="mt-3 rounded-lg border border-primary-300 px-3 py-2"
				onclick={() => retry++}>{t('statistics.twoThirds.retry')}</button
			>
		</div>
	{:else if filteredRows.length === 0}
		<p class="p-8 text-center">{t('statistics.chartControl.noData')}</p>
	{:else}
		<CustomBarChart
			data={chartRows}
			height={520}
			metricLabel={title}
			selectedCategory={mode}
			chartDescription={t(
				isParty
					? 'statistics.twoThirds.byPartyDescription'
					: 'statistics.twoThirds.byTopicDescription'
			)}
			series={segments}
			valueDomain={showPercent ? [0, 100] : undefined}
			valueSuffix={showPercent ? '%' : ''}
			valuePrecision={showPercent ? 1 : 0}
		/>
	{/if}
</section>
