<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { onMount, tick, untrack } from 'svelte';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import MultiSelectFilter from '$lib/components/Filtering/MultiSelectFilter.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import type { GenericFilterGroup } from '$lib/components/Filtering/types';
	import type { StatisticsData } from '$lib/types';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { partyToColor } from '$lib/partyColor';
	import CustomBarChart from './charts/CustomBarChart.svelte';
	import CustomDonutChart from './charts/CustomDonutChart.svelte';
	import CustomLineChart from './charts/CustomLineChart.svelte';
	import PoliticalSpectrumChart from './PoliticalSpectrumChart.svelte';

	type ChartMode = 'bar' | 'donut' | 'line' | 'spectrum';
	type CategoryOption = {
		value: string;
		label: string;
	};
	type ChartModeOption = {
		value: ChartMode;
		label: string;
		title: string;
	};

	interface Props {
		makeRequest: (
			gp: string | null,
			gender: string | null,
			isDesc: boolean,
			normalized: boolean,
			chartMode?: ChartMode
		) => Promise<StatisticsData[]>;
		height?: number;
		selectedCategory?: string;
		valueLabel?: string;
		normalizedValueLabel?: string;
		infoQuestion?: string | null;
		infoAnswer?: string | null;
		filterConfig?: {
			showNormalized?: boolean;
			showPeriod?: boolean;
			showGender?: boolean;
			showParty?: boolean;
		};
		categoryOptions?: CategoryOption[];
		chartDescriptions?: Record<string, string>;
		reloadKey?: unknown;
		showSpectrumMode?: boolean;
		selectedChartMode?: ChartMode;
		extraReservedHeight?: number;
	}

	const defaultCategoryOptions: CategoryOption[] = [
		{ value: 'delegate', label: t('common.delegates') },
		{ value: 'party', label: t('statistics.party') },
		{ value: 'gender', label: t('statistics.gender') },
		{ value: 'age', label: t('statistics.age') },
		{ value: 'legis', label: t('statistics.legis') }
	];

	let {
		makeRequest,
		height = 480,
		selectedCategory = $bindable('delegate'),
		valueLabel = t('statistics.valueLabel'),
		normalizedValueLabel = t('statistics.normalizedValueLabel'),
		infoQuestion = null,
		infoAnswer = null,
		filterConfig = {
			showNormalized: true,
			showPeriod: true,
			showGender: true,
			showParty: true
		},
		categoryOptions = defaultCategoryOptions,
		chartDescriptions = {},
		reloadKey = null,
		showSpectrumMode = false,
		selectedChartMode = $bindable<ChartMode>('bar'),
		extraReservedHeight = 0
	}: Props = $props();

	const topOptions = [
		{ value: 10, label: t('statistics.top10') },
		{ value: 25, label: t('statistics.top25') },
		{ value: 50, label: t('statistics.top50') },
		{ value: 0, label: t('statistics.all') }
	];
	const chartModeOptions: ChartModeOption[] = [
		{ value: 'bar', label: t('statistics.chartBar'), title: t('statistics.chartBarTitle') },
		{ value: 'donut', label: t('statistics.chartDonut'), title: t('statistics.chartDonutTitle') },
		{ value: 'line', label: t('statistics.chartLine'), title: t('statistics.chartLineTitle') },
		{
			value: 'spectrum',
			label: t('statistics.chartSpectrum'),
			title: t('statistics.chartSpectrumTitle')
		}
	];
	const periodOrder = ['XX', 'XXI', 'XXII', 'XXIII', 'XXIV', 'XXV', 'XXVI', 'XXVII', 'XXVIII'];

	let currentData: StatisticsData[] = $state([]);
	let currentDataCategory: string | null = $state(null);
	let loading = $state(false);
	let error: string | null = $state(null);
	let searchValue = $state('');
	let selectedParties = $state<string[]>([]);
	let topLimit = $state(25);
	let controlsHeight = $state(0);
	let windowHeight = $state(820);
	let mounted = false;
	let requestId = 0;
	let previousSelectedCategory = selectedCategory;

	let legisPeriodFilter = $state({
		title: t('statistics.legislature'),
		activeValue: 'XXVIII',
		hidden: false,
		options: [{ title: t('statistics.all'), value: 'all' }]
	});

	let genericFilters: [
		GenericFilterGroup<string>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>
	] = $state([
		{
			title: t('statistics.gender'),
			activeValue: 'all',
			hidden: false,
			options: [
				{ title: t('statistics.all'), value: 'all' },
				{ title: t('statistics.male'), value: 'm' },
				{ title: t('statistics.female'), value: 'f' }
			]
		},
		{
			title: t('statistics.sorting'),
			activeValue: 'desc',
			hidden: false,
			options: [
				{ title: t('statistics.descending'), value: 'desc' },
				{ title: t('statistics.ascending'), value: 'asc' }
			]
		},
		{
			title: t('statistics.normalization'),
			activeValue: 'normalized',
			hidden: false,
			options: [
				{ title: t('statistics.normalized'), value: 'normalized' },
				{ title: t('statistics.absolute'), value: 'absolute' }
			]
		}
	]);

	let windowWidth = $state(1024);
	let isMobile = $derived(windowWidth < 720);
	let selectedGender = $derived(genericFilters[0].activeValue);
	let isDesc = $derived(genericFilters[1].activeValue !== 'asc');
	let normalized = $derived(genericFilters[2].activeValue !== 'absolute');
	let canUsePartyFilter = $derived(
		filterConfig.showParty !== false && selectedCategory === 'delegate'
	);
	let canUseLineChart = $derived(selectedCategory === 'legis');
	let chartMode: ChartMode = $derived.by((): ChartMode => {
		if (selectedChartMode === 'line' && !canUseLineChart) return 'bar';
		if (selectedChartMode === 'spectrum' && !showSpectrumMode) return 'bar';
		return selectedChartMode;
	});
	let metricLabel = $derived(
		chartMode === 'spectrum'
			? 'Politisches Spektrum'
			: normalized
				? normalizedValueLabel
				: valueLabel
	);
	let canUseTopLimit = $derived(selectedCategory === 'delegate' && chartMode !== 'line');
	let availableChartModeOptions = $derived(
		chartModeOptions.filter((option) => {
			if (option.value === 'line') return canUseLineChart;
			if (option.value === 'spectrum') return showSpectrumMode;
			return true;
		})
	);
	let responsiveChartHeight = $derived.by(() => {
		const reservedSpace = isMobile ? 300 : 250;
		const availableHeight = windowHeight - controlsHeight - reservedSpace - extraReservedHeight;
		const maximumHeight = Math.min(height, windowHeight >= 1050 ? 820 : 720);
		return Math.round(Math.max(360, Math.min(maximumHeight, availableHeight)));
	});

	let activeCategoryLabel = $derived(
		categoryOptions.find((option) => option.value === selectedCategory)?.label ??
			t('common.delegates')
	);

	function descriptionFor(key: string) {
		const mode = normalized ? 'normalized' : 'absolute';
		return chartDescriptions[`${key}.${mode}`] ?? chartDescriptions[key];
	}

	let chartDescription = $derived.by(() => {
		if (chartMode === 'line') {
			return (
				descriptionFor('line') ??
				descriptionFor('legis') ??
				t('statistics.chartDescription.development')
			);
		}
		if (chartMode === 'spectrum') {
			return descriptionFor('spectrum') ?? t('statistics.chartDescription.spectrum');
		}
		if (chartMode === 'donut') {
			return descriptionFor('donut') ?? t('statistics.chartDescription.shares');
		}
		const categoryDescription = descriptionFor(selectedCategory);
		if (categoryDescription) {
			return categoryDescription;
		}
		if (selectedCategory === 'delegate') {
			return t('statistics.valuesOfDelegates');
		}
		if (selectedCategory === 'age') {
			return t('statistics.chartDescription.age');
		}
		if (selectedCategory === 'legis') {
			return t('statistics.chartDescription.legis');
		}
		return t('statistics.chartDescription.groups');
	});

	let selectedGp = $derived(
		filterConfig.showPeriod === false ||
			selectedCategory === 'legis' ||
			legisPeriodFilter.activeValue === 'all'
			? null
			: (legisPeriodFilter.activeValue ?? null)
	);
	let genderFilter = $derived(
		filterConfig.showGender === false || selectedCategory === 'gender' || selectedGender === 'all'
			? null
			: (selectedGender ?? null)
	);

	function colorForParty(party: string | undefined) {
		return partyToColor(party ?? null);
	}

	function colorForCategory(label: string) {
		if (selectedCategory === 'party') {
			return partyToColor(label);
		}

		const categoryColors: Record<string, string> = {
			m: '#4f46e5',
			f: '#db2777',
			XX: '#64748b',
			XXI: '#0891b2',
			XXII: '#0d9488',
			XXIII: '#65a30d',
			XXIV: '#ca8a04',
			XXV: '#ea580c',
			XXVI: '#dc2626',
			XXVII: '#9333ea',
			XXVIII: '#2563eb'
		};
		return categoryColors[label] ?? '#6b7280';
	}

	function romanToNumber(value: string) {
		const romanValues: Record<string, number> = {
			I: 1,
			V: 5,
			X: 10,
			L: 50,
			C: 100,
			D: 500,
			M: 1000
		};
		let total = 0;
		let previous = 0;
		for (const char of value.toUpperCase().split('').reverse()) {
			const current = romanValues[char];
			if (!current) return null;
			total += current < previous ? -current : current;
			previous = current;
		}
		return total;
	}

	function periodRank(gp: string) {
		const knownIndex = periodOrder.indexOf(gp);
		return knownIndex === -1 ? (romanToNumber(gp) ?? Number.MIN_SAFE_INTEGER) : knownIndex;
	}

	let displayData: StatisticsData[] = $derived.by((): StatisticsData[] => {
		if (currentDataCategory !== selectedCategory) return [];
		return currentData;
	});

	let uniqueParties = $derived.by((): { name: string; color: string }[] => {
		const parties = new Set<string>();
		for (const item of displayData) {
			const filterParty = item.partyFilter ?? item.party;
			if (item.type === 'delegate' && filterParty) parties.add(filterParty);
		}
		return [...parties]
			.sort((a, b) => a.localeCompare(b, 'de-AT'))
			.map((party) => ({
				name: party,
				color: colorForParty(party)
			}));
	});

	let filteredData = $derived.by(() => {
		const search = searchValue.trim().toLowerCase();
		return displayData
			.filter((item) => {
				const filterParty = item.partyFilter ?? item.party;
				if (
					canUsePartyFilter &&
					selectedParties.length > 0 &&
					!selectedParties.includes(filterParty ?? '')
				) {
					return false;
				}
				if (!search) return true;
				return `${item.label} ${item.party ?? ''} ${filterParty ?? ''}`
					.toLowerCase()
					.includes(search);
			})
			.sort((a, b) => (isDesc ? b.value - a.value : a.value - b.value));
	});

	let shownData = $derived(
		!canUseTopLimit || topLimit === 0 ? filteredData : filteredData.slice(0, topLimit)
	);

	let chartData = $derived(
		shownData.map((item, index) => {
			const party = item.type === 'delegate' ? (item.party?.trim() ?? 'Unbekannt') : item.label;
			const color =
				item.type === 'delegate' ? colorForParty(item.party) : colorForCategory(item.label);
			return {
				category: item.label,
				chartKey:
					item.type === 'delegate' ? `${item.label}${'\u200B'.repeat(index + 1)}` : item.label,
				value: Number(item.value ?? 0),
				party,
				color,
				valueLabel: metricLabel,
				metadata: item.metadata
			};
		})
	);

	$effect(() => {
		genericFilters[0].hidden = filterConfig.showGender === false || selectedCategory === 'gender';
		genericFilters[2].hidden = filterConfig.showNormalized === false;
		if (!canUseLineChart && selectedChartMode === 'line') {
			selectedChartMode = 'bar';
		}
		if (!showSpectrumMode && selectedChartMode === 'spectrum') {
			selectedChartMode = 'bar';
		}
	});

	$effect(() => {
		if (selectedCategory === previousSelectedCategory) return;
		previousSelectedCategory = selectedCategory;
		genericFilters[0].activeValue = 'all';
		selectedParties = [];
		searchValue = '';
		if (selectedCategory === 'legis') {
			selectedChartMode = 'line';
		}
	});

	$effect(() => {
		selectedCategory;
		selectedGp;
		genderFilter;
		isDesc;
		normalized;
		chartMode;
		makeRequest;
		reloadKey;
		if (mounted) untrack(loadData);
	});

	onMount(async () => {
		mounted = true;
		const periods = await cachedAllLegisPeriods();
		if (periods && periods.length > 0) {
			const sortedPeriods = periods.slice().sort((a, b) => periodRank(b.gp) - periodRank(a.gp));
			const latestPeriod = sortedPeriods.at(0)?.gp ?? 'XXVIII';
			legisPeriodFilter.options = [
				{ title: t('statistics.all'), value: 'all' },
				...sortedPeriods.map((period) => ({ title: period.gp, value: period.gp }))
			];
			legisPeriodFilter.activeValue = latestPeriod;
		}
		await loadData();
	});

	async function loadData() {
		const currentRequestId = ++requestId;
		const requestedCategory = selectedCategory;
		loading = true;
		error = null;
		currentData = [];
		currentDataCategory = null;
		await tick();
		if (currentRequestId !== requestId) return;
		try {
			const result = await makeRequest(selectedGp, genderFilter, isDesc, normalized, chartMode);
			if (currentRequestId !== requestId) return;
			currentData = result;
			currentDataCategory = requestedCategory;
		} catch (err) {
			if (currentRequestId !== requestId) return;
			error = err instanceof Error ? err.message : t('statistics.error.load');
		} finally {
			if (currentRequestId === requestId) loading = false;
		}
	}
</script>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight} />

<div class="statistics-chart-control space-y-5">
	<section
		bind:clientHeight={controlsHeight}
		class="relative z-20 rounded-xl border border-gray-300 bg-surface-50/95 p-4 shadow-sm backdrop-blur dark:border-surface-700 dark:bg-surface-700/95"
	>
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-3 xl:flex-row xl:items-center xl:justify-between">
				<div>
					<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">
						{t('statistics.chartControl.analysis')}
					</p>
					<div
						class="mt-2 flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400"
					>
						{#each categoryOptions as option}
							<button
								type="button"
								class="rounded-lg px-3 py-1.5 text-sm font-semibold transition {selectedCategory ===
								option.value
									? 'bg-primary-300 text-black dark:bg-primary-400'
									: 'hover:bg-primary-100 dark:hover:bg-surface-500'}"
								onclick={() => {
									selectedCategory = option.value;
									searchValue = '';
									selectedParties = [];
								}}
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>

				<div class="flex flex-col gap-2 md:flex-row md:items-end">
					<div class="min-w-64 flex-1">
						<p class="mb-2 text-sm font-semibold text-gray-600 dark:text-gray-300">
							{t('statistics.chartControl.search')}
						</p>
						<SearchBar
							bind:searchValue
							placeholder={selectedCategory === 'delegate'
								? t('delegates.searchDelegates')
								: t('statistics.searchCategory')}
						/>
					</div>
					<div class="flex h-10 gap-2 text-sm">
						{#if canUsePartyFilter && uniqueParties.length > 0}
							<MultiSelectFilter
								items={uniqueParties.map((p) => ({ value: p.name, label: p.name, color: p.color }))}
								bind:value={selectedParties}
								allLabel={t('statistics.allParties')}
							>
								{#snippet itemLabel(party)}
									<div
										class="h-3 w-3 shrink-0 rounded-full"
										style="background-color: {party.color};"
									></div>
									<span class="truncate">{party.label}</span>
								{/snippet}
							</MultiSelectFilter>
						{/if}
						<GenericFilters
							bind:genericFilters
							legisPeriodFilter={filterConfig.showPeriod === false || selectedCategory === 'legis'
								? undefined
								: legisPeriodFilter}
						/>
					</div>
				</div>
			</div>

			<div
				class="flex flex-col gap-3 border-t border-gray-300 pt-4 md:flex-row md:items-center md:justify-between dark:border-surface-600"
			>
				<div class="flex flex-wrap items-center gap-2 text-sm text-gray-700 dark:text-gray-200">
					{#if loading}
						<span>{t('statistics.chartControl.loading')}</span>
						<span class="rounded-lg bg-surface-200 px-2 py-1 font-semibold dark:bg-surface-600"
							>{activeCategoryLabel}</span
						>
					{:else}
						<span class="font-semibold">{filteredData.length}</span>
						<span>{t('statistics.chartControl.entries')}</span>
						<span class="rounded-lg bg-surface-200 px-2 py-1 font-semibold dark:bg-surface-600"
							>{activeCategoryLabel}</span
						>
						<span>{metricLabel}</span>
					{/if}
				</div>
				<div class="flex flex-wrap items-center gap-2">
					<div
						class="flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400"
						aria-label={t('statistics.chartControl.chartMode')}
					>
						{#each availableChartModeOptions as option}
							<button
								type="button"
								title={option.title}
								aria-label={option.label}
								aria-pressed={selectedChartMode === option.value}
								class="grid h-9 w-9 place-items-center rounded-lg transition {chartMode ===
								option.value
									? 'bg-primary-300 text-black dark:bg-primary-400'
									: 'text-gray-700 hover:bg-primary-100 dark:text-gray-100 dark:hover:bg-surface-500'}"
								onclick={() => (selectedChartMode = option.value)}
							>
								{#if option.value === 'bar'}
									<svg
										viewBox="0 0 24 24"
										class="h-5 w-5"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										aria-hidden="true"
									>
										<title>{option.title}</title>
										<path d="M4 19V5" />
										<path d="M4 19h16" />
										<path d="M8 16h9" />
										<path d="M8 12h6" />
										<path d="M8 8h11" />
									</svg>
								{:else if option.value === 'donut'}
									<svg
										viewBox="0 0 24 24"
										class="h-5 w-5"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										aria-hidden="true"
									>
										<title>{option.title}</title>
										<path d="M12 3a9 9 0 1 1-8.49 6" />
										<path d="M12 3v6h6" />
										<circle cx="12" cy="12" r="3" />
									</svg>
								{:else if option.value === 'line'}
									<svg
										viewBox="0 0 24 24"
										class="h-5 w-5"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										aria-hidden="true"
									>
										<title>{option.title}</title>
										<path d="M4 19V5" />
										<path d="M4 19h16" />
										<path d="m7 15 4-5 3 3 5-7" />
									</svg>
								{:else}
									<svg
										viewBox="0 0 24 24"
										class="h-5 w-5"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										aria-hidden="true"
									>
										<title>{option.title}</title>
										<path d="M12 4v16" />
										<path d="M4 12h16" />
										<circle cx="15.5" cy="8.5" r="1.5" />
										<circle cx="8.5" cy="15.5" r="1.5" />
										<circle cx="14" cy="14" r="1.5" />
									</svg>
								{/if}
							</button>
						{/each}
					</div>

					{#if canUseTopLimit}
						<div
							class="flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400"
						>
							{#each topOptions as option}
								<button
									type="button"
									class="rounded-lg px-3 py-1.5 text-sm font-semibold transition {topLimit ===
									option.value
										? 'bg-primary-300 text-black dark:bg-primary-400'
										: 'hover:bg-primary-100 dark:hover:bg-surface-500'}"
									onclick={() => (topLimit = option.value)}
								>
									{option.label}
								</button>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<section
		class="relative z-0 rounded-xl border border-gray-300 bg-white shadow-sm dark:border-surface-700 dark:bg-surface-800"
	>
		{#if chartMode !== 'bar'}
			<div
				class="flex flex-col gap-2 border-b border-gray-200 p-4 md:flex-row md:items-start md:justify-between dark:border-surface-700"
			>
				<div>
					<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">{metricLabel}</h2>
					<p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
						{chartDescription}
					</p>
				</div>
				{#if infoQuestion && infoAnswer}
					<div class="group relative">
						<button
							type="button"
							class="rounded-lg border border-primary-300 px-3 py-1.5 text-sm font-semibold hover:bg-primary-100 dark:border-primary-400 dark:hover:bg-surface-700"
						>
							{infoQuestion}
						</button>
						<div
							class="invisible absolute top-10 right-0 z-30 w-80 rounded-xl border border-gray-300 bg-surface-50 p-4 text-sm opacity-0 shadow-lg transition group-hover:visible group-hover:opacity-100 dark:border-surface-600 dark:bg-surface-700"
						>
							<div class="space-y-2 text-gray-700 dark:text-gray-100">
								{@html infoAnswer}
							</div>
						</div>
					</div>
				{/if}
			</div>
		{/if}

		{#if loading}
			<div class="flex min-h-80 items-center justify-center p-8">
				<div
					class="h-10 w-10 animate-spin rounded-full border-4 border-surface-200 border-t-primary-500 dark:border-surface-700"
				></div>
			</div>
		{:else if error}
			<div class="flex min-h-80 flex-col items-center justify-center gap-3 p-8 text-center">
				<p class="font-semibold text-red-700 dark:text-red-300">
					{t('statistics.chartControl.errorTitle')}
				</p>
				<p class="max-w-lg text-sm text-red-600 dark:text-red-200">{error}</p>
				<button
					type="button"
					class="rounded-lg bg-primary-500 px-4 py-2 text-sm font-semibold text-white hover:bg-primary-600"
					onclick={loadData}
				>
					{t('statistics.chartControl.retry')}
				</button>
			</div>
		{:else if chartData.length === 0}
			<div class="flex min-h-80 flex-col items-center justify-center gap-2 p-8 text-center">
				<p class="font-semibold text-gray-800 dark:text-gray-100">
					{t('statistics.chartControl.noData')}
				</p>
				<p class="text-sm text-gray-600 dark:text-gray-300">
					{t('statistics.chartControl.noDataHint')}
				</p>
			</div>
		{:else if chartMode === 'spectrum'}
			<PoliticalSpectrumChart data={shownData} height={responsiveChartHeight} {selectedCategory} />
		{:else if chartMode === 'donut'}
			<CustomDonutChart data={chartData} height={responsiveChartHeight} {metricLabel} />
		{:else if chartMode === 'line'}
			<CustomLineChart data={chartData} height={responsiveChartHeight} {selectedCategory} />
		{:else}
			<CustomBarChart
				data={chartData}
				height={responsiveChartHeight}
				{metricLabel}
				{selectedCategory}
				{chartDescription}
				{infoQuestion}
				{infoAnswer}
			/>
		{/if}
	</section>
</div>

<style>
	:global(.layerchart) {
		font-family: inherit;
	}
</style>
