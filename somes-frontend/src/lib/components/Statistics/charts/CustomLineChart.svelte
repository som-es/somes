<script lang="ts">
	import { LineChart } from 'layerchart';
	import { partyColors } from '$lib/partyColor';

	type ChartItem = {
		category: string;
		value: number;
		party: string;
		color: string;
		valueLabel: string;
		metadata?: Record<string, any>;
	};

	let {
		data,
		height = 520,
		selectedCategory
	}: {
		data: ChartItem[];
		height?: number;
		selectedCategory: string;
	} = $props();

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
