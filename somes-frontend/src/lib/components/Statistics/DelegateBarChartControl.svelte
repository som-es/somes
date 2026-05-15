<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import { BarChart, LineChart, PieChart } from 'layerchart';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import MultiValuesFilter from '$lib/components/Filtering/MultiValuesFilter.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import type { GenericFilterGroup } from '$lib/components/Filtering/types';
	import type { StatisticsData } from '$lib/types';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { partyToColor } from '$lib/partyColor';

	type ChartMode = 'bar' | 'donut' | 'line';
	type CategoryOption = {
		value: string;
		label: string;
	};

	interface Props {
		delegateMakeRequest: (
			gp: string | null,
			gender: string | null,
			isDesc: boolean,
			normalized: boolean
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
	}

	let {
		delegateMakeRequest,
		height = 480,
		selectedCategory = 'delegate',
		valueLabel = 'Wert',
		normalizedValueLabel = 'Wert (normalisiert)',
		infoQuestion = null,
		infoAnswer = null,
		filterConfig = {
			showNormalized: true,
			showPeriod: true,
			showGender: true,
			showParty: true
		}
	}: Props = $props();

	const categoryOptions: CategoryOption[] = [
		{ value: 'delegate', label: 'Abgeordnete' },
		{ value: 'party', label: 'Parteien' },
		{ value: 'gender', label: 'Geschlecht' },
		{ value: 'age', label: 'Alter' },
		{ value: 'legis', label: 'Legislaturperioden' }
	];

	const topOptions = [
		{ value: 10, label: 'Top 10' },
		{ value: 25, label: 'Top 25' },
		{ value: 50, label: 'Top 50' },
		{ value: 0, label: 'Alle' }
	];
	const periodOrder = ['XX', 'XXI', 'XXII', 'XXIII', 'XXIV', 'XXV', 'XXVI', 'XXVII', 'XXVIII'];

	let currentData: StatisticsData[] = $state([]);
	let loading = $state(false);
	let error: string | null = $state(null);
	let searchValue = $state('');
	let selectedParties = $state(new SvelteSet<string>());
	let topLimit = $state(25);
	let mounted = false;

	let legisPeriodFilter = $state({
		title: 'Legislaturperiode',
		activeValue: 'XXVIII',
		hidden: false,
		options: [{ title: 'Alle', value: 'all' }]
	});

	let genericFilters: [
		GenericFilterGroup<string>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>
	] = $state([
		{
			title: 'Geschlecht',
			activeValue: 'all',
			hidden: false,
			options: [
				{ title: 'Alle', value: 'all' },
				{ title: 'Männlich', value: 'm' },
				{ title: 'Weiblich', value: 'f' }
			]
		},
		{
			title: 'Sortierung',
			activeValue: 'desc',
			hidden: false,
			options: [
				{ title: 'Absteigend', value: 'desc' },
				{ title: 'Aufsteigend', value: 'asc' }
			]
		},
		{
			title: 'Normalisierung',
			activeValue: 'normalized',
			hidden: false,
			options: [
				{ title: 'Normalisiert', value: 'normalized' },
				{ title: 'Absolut', value: 'absolute' }
			]
		},
		{
			title: 'Darstellung',
			activeValue: 'bar',
			hidden: false,
			options: [
				{ title: 'Balken', value: 'bar' },
				{ title: 'Anteile', value: 'donut' },
				{ title: 'Verlauf', value: 'line' }
			]
		}
	]);

	let windowWidth = $state(1024);
	let isMobile = $derived(windowWidth < 720);
	let selectedGender = $derived(genericFilters[0].activeValue);
	let isDesc = $derived(genericFilters[1].activeValue !== 'asc');
	let normalized = $derived(genericFilters[2].activeValue !== 'absolute');
	let requestedChartMode = $derived(genericFilters[3].activeValue as ChartMode);
	let metricLabel = $derived(normalized ? normalizedValueLabel : valueLabel);
	let canUsePartyFilter = $derived(
		filterConfig.showParty !== false && selectedCategory === 'delegate'
	);
	let canUseLineChart = $derived(selectedCategory === 'legis');
	let chartMode = $derived(
		requestedChartMode === 'line' && !canUseLineChart ? 'bar' : requestedChartMode
	);

	let activeCategoryLabel = $derived(
		categoryOptions.find((option) => option.value === selectedCategory)?.label ?? 'Abgeordnete'
	);

	let selectedGp = $derived(
		selectedCategory === 'legis' || legisPeriodFilter.activeValue === 'all'
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

	let uniqueParties = $derived.by(() => {
		const parties = new Set<string>();
		for (const item of currentData) {
			if (item.type === 'delegate' && item.party) parties.add(item.party);
		}
		return [...parties].sort((a, b) => a.localeCompare(b, 'de-AT'));
	});

	let filteredData = $derived.by(() => {
		const search = searchValue.trim().toLowerCase();
		return currentData
			.filter((item) => {
				if (
					canUsePartyFilter &&
					selectedParties.size > 0 &&
					!selectedParties.has(item.party ?? '')
				) {
					return false;
				}
				if (!search) return true;
				return `${item.label} ${item.party ?? ''}`.toLowerCase().includes(search);
			})
			.sort((a, b) => (isDesc ? b.value - a.value : a.value - b.value));
	});

	let shownData = $derived(topLimit === 0 ? filteredData : filteredData.slice(0, topLimit));

	let chartData = $derived(
		shownData.map((item) => {
			const party = item.type === 'delegate' ? (item.party?.trim() ?? 'Unbekannt') : item.label;
			return {
				category: item.label,
				value: Number(item.value ?? 0),
				party,
				color: item.type === 'delegate' ? colorForParty(item.party) : colorForCategory(item.label),
				valueLabel: metricLabel,
				metadata: item.metadata
			};
		})
	);

	let donutData = $derived.by(() => {
		const source = chartData.slice(0, 12);
		const rest = chartData.slice(12);
		const restValue = rest.reduce((sum, item) => sum + item.value, 0);
		const items = source.map((item) => ({
			key: item.category,
			label: item.category,
			value: item.value,
			party: item.party,
			color: item.color
		}));
		if (restValue > 0) {
			items.push({
				key: 'Weitere',
				label: 'Weitere',
				value: restValue,
				party: 'Weitere',
				color: '#94a3b8'
			});
		}
		return items;
	});

	let lineData = $derived(
		[...chartData]
			.sort((a, b) => periodOrder.indexOf(a.category) - periodOrder.indexOf(b.category))
			.map((item) => ({
				period: item.category,
				value: item.value,
				party: item.party
			}))
	);

	let cRange = $derived(chartData.map((item) => item.color));
	let donutRange = $derived(donutData.map((item) => item.color));
	let chartHeight = $derived(
		chartMode === 'bar' ? Math.max(height, chartData.length * (isMobile ? 30 : 34) + 80) : height
	);
	let chartPaddingLeft = $derived(isMobile ? 150 : selectedCategory === 'delegate' ? 285 : 190);

	$effect(() => {
		if (selectedCategory === 'legis') {
			legisPeriodFilter.activeValue = 'all';
		}
		genericFilters[0].hidden = filterConfig.showGender === false || selectedCategory === 'gender';
		genericFilters[2].hidden = filterConfig.showNormalized === false;
		genericFilters[3].options = canUseLineChart
			? [
					{ title: 'Balken', value: 'bar' },
					{ title: 'Anteile', value: 'donut' },
					{ title: 'Verlauf', value: 'line' }
				]
			: [
					{ title: 'Balken', value: 'bar' },
					{ title: 'Anteile', value: 'donut' }
				];
		if (!canUseLineChart && genericFilters[3].activeValue === 'line') {
			genericFilters[3].activeValue = 'bar';
		}
	});

	$effect(() => {
		selectedCategory;
		selectedGp;
		genderFilter;
		isDesc;
		normalized;
		delegateMakeRequest;
		if (mounted) untrack(loadData);
	});

	onMount(async () => {
		mounted = true;
		const periods = await cachedAllLegisPeriods();
		if (periods && periods.length > 0) {
			legisPeriodFilter.options = [
				{ title: 'Alle', value: 'all' },
				...periods.map((period) => ({ title: period.gp, value: period.gp }))
			];
			legisPeriodFilter.activeValue = periods.at(-1)?.gp ?? 'XXVIII';
		}
		await loadData();
	});

	async function loadData() {
		loading = true;
		error = null;
		try {
			const result = await delegateMakeRequest(selectedGp, genderFilter, isDesc, normalized);
			currentData = result;
		} catch (err) {
			error =
				err instanceof Error ? err.message : 'Die Statistikdaten konnten nicht geladen werden.';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:window bind:innerWidth={windowWidth} />

<div class="space-y-5">
	<section
		class="rounded-xl border border-gray-300 bg-surface-50 p-4 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
	>
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-3 xl:flex-row xl:items-center xl:justify-between">
				<div>
					<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">Auswertung</p>
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
									selectedParties.clear();
								}}
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>

				<div class="flex flex-col gap-2 md:flex-row md:items-end">
					<div class="min-w-64 flex-1">
						<p class="mb-2 text-sm font-semibold text-gray-600 dark:text-gray-300">Suche</p>
						<SearchBar
							bind:searchValue
							placeholder={selectedCategory === 'delegate'
								? 'Abgeordnete suchen...'
								: 'Kategorie suchen...'}
						/>
					</div>
					<div class="flex h-10 gap-2 text-sm">
						{#if canUsePartyFilter && uniqueParties.length > 0}
							<MultiValuesFilter
								title="Parteien"
								bind:selectedValues={selectedParties}
								values={uniqueParties}
							/>
						{/if}
						<GenericFilters
							bind:genericFilters
							legisPeriodFilter={filterConfig.showPeriod === false ? undefined : legisPeriodFilter}
						/>
					</div>
				</div>
			</div>

			<div
				class="flex flex-col gap-3 border-t border-gray-300 pt-4 md:flex-row md:items-center md:justify-between dark:border-surface-600"
			>
				<div class="flex flex-wrap items-center gap-2 text-sm text-gray-700 dark:text-gray-200">
					<span class="font-semibold">{filteredData.length}</span>
					<span>Einträge in</span>
					<span class="rounded-lg bg-surface-200 px-2 py-1 font-semibold dark:bg-surface-600"
						>{activeCategoryLabel}</span
					>
					<span>{metricLabel}</span>
				</div>
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
			</div>
		</div>
	</section>

	<section
		class="rounded-xl border border-gray-300 bg-white shadow-sm dark:border-surface-700 dark:bg-surface-800"
	>
		<div
			class="flex flex-col gap-2 border-b border-gray-200 p-4 md:flex-row md:items-start md:justify-between dark:border-surface-700"
		>
			<div>
				<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">{metricLabel}</h2>
				<p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
					{chartMode === 'bar'
						? 'Horizontale Balken halten lange Namen lesbar.'
						: chartMode === 'line'
							? 'Verlauf über die Legislaturperioden.'
							: 'Anteile der größten Einträge, übrige Werte zusammengefasst.'}
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

		{#if loading}
			<div class="flex min-h-80 items-center justify-center p-8">
				<div
					class="h-10 w-10 animate-spin rounded-full border-4 border-surface-200 border-t-primary-500 dark:border-surface-700"
				></div>
			</div>
		{:else if error}
			<div class="flex min-h-80 flex-col items-center justify-center gap-3 p-8 text-center">
				<p class="font-semibold text-red-700 dark:text-red-300">Fehler beim Laden der Daten</p>
				<p class="max-w-lg text-sm text-red-600 dark:text-red-200">{error}</p>
				<button
					type="button"
					class="rounded-lg bg-primary-500 px-4 py-2 text-sm font-semibold text-white hover:bg-primary-600"
					onclick={loadData}
				>
					Erneut versuchen
				</button>
			</div>
		{:else if chartData.length === 0}
			<div class="flex min-h-80 flex-col items-center justify-center gap-2 p-8 text-center">
				<p class="font-semibold text-gray-800 dark:text-gray-100">Keine Daten gefunden</p>
				<p class="text-sm text-gray-600 dark:text-gray-300">
					Passen Sie Suche, Parteien oder Filter an.
				</p>
			</div>
		{:else if chartMode === 'donut'}
			<div
				class="grid gap-4 p-4 lg:grid-cols-[minmax(0,1fr)_20rem]"
				style="min-height: {height}px;"
			>
				<div class="h-[420px] min-w-0">
					<PieChart
						data={donutData}
						key="key"
						label="label"
						value="value"
						c="key"
						cRange={donutRange}
						innerRadius={0.52}
						cornerRadius={3}
						padAngle={1}
					/>
				</div>
				<div
					class="max-h-[420px] overflow-y-auto rounded-lg border border-gray-200 p-3 dark:border-surface-700"
				>
					{#each donutData as item}
						<div
							class="flex items-center gap-2 border-b border-gray-100 py-2 last:border-0 dark:border-surface-700"
						>
							<span class="h-3 w-3 shrink-0 rounded-full" style="background-color: {item.color}"
							></span>
							<span class="min-w-0 flex-1 truncate text-sm font-medium">{item.label}</span>
							<span class="text-sm text-gray-600 tabular-nums dark:text-gray-300"
								>{item.value.toFixed(item.value < 10 ? 2 : 0)}</span
							>
						</div>
					{/each}
				</div>
			</div>
		{:else if chartMode === 'line'}
			<div class="p-4" style="height: {height}px;">
				<LineChart
					data={lineData}
					x="period"
					y="value"
					c="party"
					{cRange}
					padding={{ left: 64, right: 24, top: 24, bottom: 48 }}
					props={{
						xAxis: {
							tickLabelProps: {
								class: 'fill-black dark:fill-white stroke-none text-xs font-semibold'
							}
						},
						yAxis: {
							tickLabelProps: {
								class: 'fill-black dark:fill-white stroke-none text-xs font-semibold'
							}
						}
					}}
				/>
			</div>
		{:else}
			<div class="overflow-x-auto">
				<div class="min-w-[760px] p-4" style="height: {chartHeight}px;">
					<BarChart
						data={chartData}
						x="value"
						y="category"
						c="party"
						{cRange}
						orientation="horizontal"
						padding={{ left: chartPaddingLeft, right: 36, top: 24, bottom: 32 }}
						props={{
							xAxis: {
								tickLabelProps: {
									class: 'fill-black dark:fill-white stroke-none text-xs font-semibold'
								}
							},
							yAxis: {
								tickLabelProps: {
									class: 'fill-black dark:fill-white stroke-none font-semibold',
									'font-size': isMobile ? 9 : 11,
									textAnchor: 'end'
								}
							},
							bars: {
								strokeWidth: 0,
								rx: 3
							}
						}}
					/>
				</div>
			</div>
		{/if}
	</section>
</div>

<style>
	:global(.layerchart) {
		font-family: inherit;
	}
</style>
