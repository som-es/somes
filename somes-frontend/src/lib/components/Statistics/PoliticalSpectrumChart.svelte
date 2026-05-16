<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { StatisticsData } from '$lib/types';

	let {
		data,
		height = 520,
		selectedCategory
	}: {
		data: StatisticsData[];
		height?: number;
		selectedCategory: string;
	} = $props();

	const chartSize = 520;
	const padding = 56;
	const plotSize = chartSize - padding * 2;
	const center = chartSize / 2;

	function clamp(value: number, min: number, max: number) {
		return Math.min(max, Math.max(min, value));
	}

	function pointColor(item: StatisticsData) {
		return item.type === 'delegate' ? partyToColor(item.party ?? null) : partyToColor(item.label);
	}

	function pointX(item: StatisticsData) {
		return Number(item.metadata?.left_right_score ?? 0);
	}

	function pointY(item: StatisticsData) {
		return Number(item.metadata?.liberal_authoritarian_score ?? 0);
	}

	let domain = $derived.by(() => {
		const maxValue = data.reduce(
			(max, item) => Math.max(max, Math.abs(pointX(item)), Math.abs(pointY(item))),
			0
		);
		return Math.max(0.1, maxValue);
	});

	let points = $derived(
		data.map((item) => {
			const x = pointX(item);
			const y = pointY(item);
			const screenX = center + clamp(x / domain, -1, 1) * (plotSize / 2);
			const screenY = center - clamp(y / domain, -1, 1) * (plotSize / 2);
			return {
				item,
				x,
				y,
				screenX,
				screenY,
				color: pointColor(item)
			};
		})
	);

	let sideItems = $derived(points.slice(0, selectedCategory === 'delegate' ? 12 : points.length));
</script>

<div class="grid gap-4 p-4 lg:grid-cols-[minmax(0,1fr)_20rem]" style="min-height: {height}px;">
	<div class="flex min-w-0 items-center justify-center">
		<div class="w-full max-w-[680px]">
			<svg
				viewBox="0 0 {chartSize} {chartSize}"
				class="aspect-square w-full rounded-lg border border-gray-200 bg-white shadow-inner dark:border-surface-700 dark:bg-surface-900"
				role="img"
				aria-label="Politisches Spektrum"
			>
				<rect
					x={padding}
					y={padding}
					width={plotSize}
					height={plotSize}
					fill="none"
					stroke="currentColor"
					class="text-gray-300 dark:text-surface-600"
				/>
				<line
					x1={center}
					y1={padding}
					x2={center}
					y2={chartSize - padding}
					stroke="currentColor"
					stroke-width="1.5"
					class="text-gray-300 dark:text-surface-600"
				/>
				<line
					x1={padding}
					y1={center}
					x2={chartSize - padding}
					y2={center}
					stroke="currentColor"
					stroke-width="1.5"
					class="text-gray-300 dark:text-surface-600"
				/>

				<text
					x={padding - 12}
					y={center}
					text-anchor="middle"
					transform="rotate(-90 {padding - 12} {center})"
					class="fill-gray-700 text-[12px] font-bold dark:fill-gray-200"
				>
					SOZIALISTISCH
				</text>
				<text
					x={chartSize - padding + 12}
					y={center}
					text-anchor="middle"
					transform="rotate(-90 {chartSize - padding + 12} {center})"
					class="fill-gray-700 text-[12px] font-bold dark:fill-gray-200"
				>
					KAPITALISTISCH
				</text>
				<text
					x={center}
					y={padding - 22}
					text-anchor="middle"
					class="fill-gray-700 text-[12px] font-bold dark:fill-gray-200"
				>
					AUTORITÄR
				</text>
				<text
					x={center}
					y={chartSize - padding + 34}
					text-anchor="middle"
					class="fill-gray-700 text-[12px] font-bold dark:fill-gray-200"
				>
					LIBERTÄR
				</text>

				{#each points as point}
					<circle
						cx={point.screenX}
						cy={point.screenY}
						r={selectedCategory === 'delegate' ? 5 : 8}
						fill={point.color}
						stroke="white"
						stroke-width="1.5"
						class="drop-shadow-sm"
					>
						<title
							>{point.item.label}: links/rechts {point.x.toFixed(3)}, libertär/autoritär {point.y.toFixed(
								3
							)}</title
						>
					</circle>
				{/each}
			</svg>
		</div>
	</div>

	<div
		class="max-h-[520px] overflow-y-auto rounded-lg border border-gray-200 p-3 dark:border-surface-700"
	>
		<div class="mb-2 text-sm font-semibold text-gray-700 dark:text-gray-200">Punkte</div>
		{#each sideItems as point}
			<div class="border-b border-gray-100 py-2 last:border-0 dark:border-surface-700">
				<div class="flex items-center gap-2">
					<span class="h-3 w-3 shrink-0 rounded-full" style="background-color: {point.color}"
					></span>
					<span class="min-w-0 flex-1 truncate text-sm font-semibold">{point.item.label}</span>
				</div>
				<div class="mt-1 grid grid-cols-2 gap-2 pl-5 text-xs text-gray-600 dark:text-gray-300">
					<span class="tabular-nums">L/R {point.x.toFixed(2)}</span>
					<span class="tabular-nums">L/A {point.y.toFixed(2)}</span>
				</div>
			</div>
		{/each}
	</div>
</div>
