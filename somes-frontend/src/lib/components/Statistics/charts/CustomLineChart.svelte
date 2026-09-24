<script lang="ts">
	import { LineChart, Points, Spline } from 'layerchart';
	import { localeStore } from '$lib/i18n';
	import { partyColors } from '$lib/partyColor';

	type ChartItem = {
		category: string;
		value: number;
		party: string;
		color: string;
		valueLabel: string;
		metadata?: Record<string, unknown>;
	};

	let {
		data,
		height = 520,
		selectedCategory,
		valueDomain,
		valuePrecision
	}: {
		data: ChartItem[];
		height?: number;
		selectedCategory: string;
		valueDomain?: [number, number];
		valuePrecision?: number;
	} = $props();

	const numberFormat = $derived(
		new Intl.NumberFormat(localeStore.value, {
			maximumFractionDigits: valuePrecision ?? 2,
			minimumFractionDigits: 0
		})
	);

	const periodOrder = ['XX', 'XXI', 'XXII', 'XXIII', 'XXIV', 'XXV', 'XXVI', 'XXVII', 'XXVIII'];

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

	let lineData = $derived(
		[...data]
			.sort((a, b) => periodRank(a.category) - periodRank(b.category))
			.map((item) => ({
				period: item.category,
				value: item.value,
				party: item.party
			}))
	);

	const cRange = $derived.by(() => {
		if (selectedCategory === 'delegate') {
			const values = partyColors
				.values()
				.map((key) => key)
				.toArray();
			values.push('grey', 'grey');
			return values;
		}

		return data.map((item) => item.color);
	});
</script>

<div class="p-4">
	<dl class="mb-3 flex flex-wrap gap-x-6 gap-y-2 text-sm">
		{#each lineData as item (item.period)}
			<div class="flex gap-2">
				<dt class="text-gray-600 dark:text-gray-300">{item.period}</dt>
				<dd class="font-semibold tabular-nums">{numberFormat.format(item.value)}</dd>
			</div>
		{/each}
	</dl>
	<div style="height: {Math.max(240, height - 64)}px;">
		<LineChart
			data={lineData}
			x="period"
			y="value"
			yDomain={valueDomain}
			yBaseline={valueDomain ? null : 0}
			yNice={valueDomain ? false : undefined}
			c="party"
			{cRange}
			padding={{ left: 64, right: 24, top: 24, bottom: 48 }}
			props={{
				tooltip: { item: { format: (value: number) => numberFormat.format(value) } },
				xAxis: {
					tickLabelProps: {
						class: 'fill-black dark:fill-white stroke-none text-xs font-semibold'
					}
				},
				yAxis: {
					format: (value: number) => numberFormat.format(value),
					tickLabelProps: {
						class: 'fill-black dark:fill-white stroke-none text-xs font-semibold'
					}
				}
			}}
		>
			{#snippet marks()}
				<Spline seriesKey="default" stroke="currentColor" strokeWidth={2} />
				<Points seriesKey="default" r={4} fill="currentColor" />
			{/snippet}
		</LineChart>
	</div>
</div>
