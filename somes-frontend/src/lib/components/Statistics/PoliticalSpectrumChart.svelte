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
	const coordinateCompression = 1.6;
	const maxPlotShare = 0.82;

	let hoveredIndex = $state<number | null>(null);

	function clamp(value: number, min: number, max: number) {
		return Math.min(max, Math.max(min, value));
	}

	function compressedCoordinate(value: number, domain: number) {
		const normalizedValue = clamp(value / domain, -1, 1);
		const compressed =
			Math.tanh(normalizedValue * coordinateCompression) / Math.tanh(coordinateCompression);
		return compressed * maxPlotShare;
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
		return Math.max(0.1, maxValue * 1.2);
	});

	let points = $derived(
		data.map((item) => {
			const x = pointX(item);
			const y = pointY(item);
			const screenX = center + compressedCoordinate(x, domain) * (plotSize / 2);
			const screenY = center - compressedCoordinate(y, domain) * (plotSize / 2);
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
	let hoveredPoint = $derived(hoveredIndex === null ? null : (points[hoveredIndex] ?? null));
</script>

<div class="grid gap-4 p-4 lg:grid-cols-[minmax(0,1fr)_20rem]" style="min-height: {height}px;">
	<div class="flex min-w-0 items-center justify-center">
		<div class="relative w-full max-w-[680px]">
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

				{#each points as point, index}
					<circle
						cx={point.screenX}
						cy={point.screenY}
						r={hoveredIndex === index
							? selectedCategory === 'delegate'
								? 8
								: 11
							: selectedCategory === 'delegate'
								? 5
								: 8}
						fill={point.color}
						stroke="white"
						stroke-width="1.5"
						class="cursor-pointer drop-shadow-sm transition"
						role="img"
						aria-label="{point.item.label}: sozialistisch/kapitalistisch {point.x.toFixed(
							3
						)}, libertär/autoritär {point.y.toFixed(3)}"
						onmouseenter={() => (hoveredIndex = index)}
						onmouseleave={() => (hoveredIndex = null)}
					/>
				{/each}
			</svg>

			{#if hoveredPoint}
				<div
					class="pointer-events-none absolute z-10 max-w-64 min-w-52 rounded-lg border border-gray-200 bg-white p-3 text-sm shadow-lg dark:border-surface-700 dark:bg-surface-800"
					style="left: {clamp((hoveredPoint.screenX / chartSize) * 100, 8, 72)}%; top: {clamp(
						(hoveredPoint.screenY / chartSize) * 100,
						8,
						76
					)}%;"
				>
					<div class="flex items-center gap-2">
						<span
							class="h-3 w-3 shrink-0 rounded-full"
							style="background-color: {hoveredPoint.color}"
						></span>
						<span class="min-w-0 truncate font-semibold text-gray-900 dark:text-gray-50">
							{hoveredPoint.item.label}
						</span>
					</div>
					{#if hoveredPoint.item.party}
						<div class="mt-1 text-xs text-gray-500 dark:text-gray-400">
							{hoveredPoint.item.party}
						</div>
					{/if}
					<div class="mt-2 grid grid-cols-2 gap-2 text-xs text-gray-700 dark:text-gray-200">
						<span>Soz/Kap</span>
						<span class="text-right tabular-nums">{hoveredPoint.x.toFixed(3)}</span>
						<span>Lib/Auth</span>
						<span class="text-right tabular-nums">{hoveredPoint.y.toFixed(3)}</span>
					</div>
				</div>
			{/if}
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
					<span class="tabular-nums">S/K {point.x.toFixed(2)}</span>
					<span class="tabular-nums">L/A {point.y.toFixed(2)}</span>
				</div>
			</div>
		{/each}
	</div>
</div>
