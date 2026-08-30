<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { PieChart } from 'layerchart';

	type ChartItem = {
		category: string;
		value: number;
		party: string;
		color: string;
		valueLabel: string;
		metadata?: Record<string, any>;
	};

	type DonutItem = {
		key: string;
		label: string;
		value: number;
		party: string;
		color: string;
	};

	let {
		data,
		height = 520,
		metricLabel
	}: {
		data: ChartItem[];
		height?: number;
		metricLabel: string;
	} = $props();

	const visibleSlices = 12;

	function formatValue(value: number) {
		const abs = Math.abs(value);
		const maximumFractionDigits = abs >= 100 ? 0 : abs >= 10 ? 1 : 2;
		return new Intl.NumberFormat('de-AT', {
			maximumFractionDigits,
			minimumFractionDigits: 0
		}).format(value);
	}

	let donutData = $derived.by((): DonutItem[] => {
		const positiveItems = data
			.map((item) => ({
				key: item.category,
				label: item.category,
				value: Math.max(Number(item.value ?? 0), 0),
				party: item.party,
				color: item.color
			}))
			.filter((item) => item.value > 0);
		const source = positiveItems.slice(0, visibleSlices);
		const rest = positiveItems.slice(visibleSlices);
		const restValue = rest.reduce((sum, item) => sum + item.value, 0);

		if (restValue > 0) {
			return [
				...source,
				{
					key: 'Weitere',
					label: t('statistics.others'),
					value: restValue,
					party: 'Weitere',
					color: '#94a3b8'
				}
			];
		}

		return source;
	});

	let donutTotal = $derived(donutData.reduce((sum, item) => sum + item.value, 0));
	let largestDonutItem = $derived(
		donutData.reduce<DonutItem | null>(
			(current, item) => (!current || item.value > current.value ? item : current),
			null
		)
	);
	let colorRange = $derived(donutData.map((item) => item.color));
</script>

<div class="grid gap-4 p-4 lg:grid-cols-[minmax(0,1fr)_20rem]" style="min-height: {height}px;">
	<div class="flex min-h-[420px] min-w-0 items-center justify-center">
		<div
			class="relative aspect-square w-full max-w-[380px] [--donut-stroke:white] dark:[--donut-stroke:rgb(31_41_55)]"
		>
			{#if donutTotal > 0}
				<PieChart
					data={donutData}
					key="key"
					label="label"
					value="value"
					c="color"
					cRange={colorRange}
					innerRadius={0.62}
					cornerRadius={3}
					padAngle={0.012}
					legend={false}
					tooltip={true}
					props={{
						arc: {
							stroke: 'var(--donut-stroke)',
							strokeWidth: 2
						}
					}}
				/>
			{:else}
				<div
					class="absolute inset-0 rounded-full border border-gray-200 dark:border-surface-700"
				></div>
			{/if}
			<div
				class="pointer-events-none absolute inset-[26%] flex flex-col items-center justify-center rounded-full border border-gray-200 bg-white text-center shadow-sm dark:border-surface-700 dark:bg-surface-800"
			>
				<span class="text-xs font-semibold text-gray-500 uppercase dark:text-gray-400">Summe</span>
				<span class="mt-1 text-2xl font-bold text-gray-900 tabular-nums dark:text-gray-50">
					{formatValue(donutTotal)}
				</span>
				{#if largestDonutItem}
					<span class="mt-2 max-w-32 truncate text-xs text-gray-600 dark:text-gray-300">
						{largestDonutItem.label}
					</span>
				{/if}
			</div>
		</div>
	</div>

	<div
		class="chart-scrollbar max-h-[420px] overflow-y-auto rounded-lg border border-gray-200 p-3 dark:border-surface-700"
	>
		{#each donutData as item}
			{@const share = donutTotal > 0 ? (item.value / donutTotal) * 100 : 0}
			<div
				class="flex items-center gap-2 border-b border-gray-100 py-2 last:border-0 dark:border-surface-700"
			>
				<span class="h-3 w-3 shrink-0 rounded-full" style="background-color: {item.color}"></span>
				<span class="min-w-0 flex-1 truncate text-sm font-medium">{item.label}</span>
				<div class="text-right">
					<div class="text-sm text-gray-600 tabular-nums dark:text-gray-300">
						{formatValue(item.value)}
					</div>
					<div class="text-xs text-gray-500 tabular-nums dark:text-gray-400">
						{share.toFixed(1)}%
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.chart-scrollbar {
		scrollbar-width: thin;
		scrollbar-color: rgb(156 163 175) transparent;
	}

	.chart-scrollbar::-webkit-scrollbar {
		width: 10px;
	}

	.chart-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}

	.chart-scrollbar::-webkit-scrollbar-thumb {
		background: rgb(156 163 175);
		border: 3px solid transparent;
		border-radius: 999px;
		background-clip: content-box;
	}

	.chart-scrollbar::-webkit-scrollbar-thumb:hover {
		background: rgb(107 114 128);
		background-clip: content-box;
	}
</style>
