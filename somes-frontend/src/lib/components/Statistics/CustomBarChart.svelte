<script lang="ts">
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
		metricLabel,
		selectedCategory,
		chartDescription,
		stickyTopOffset = 0,
		infoQuestion = null,
		infoAnswer = null
	}: {
		data: ChartItem[];
		height?: number;
		metricLabel: string;
		selectedCategory: string;
		chartDescription: string;
		stickyTopOffset?: number;
		infoQuestion?: string | null;
		infoAnswer?: string | null;
	} = $props();

	let hoveredIndex = $state<number | null>(null);

	const labelColumnWidth = $derived(selectedCategory === 'delegate' ? '17rem' : '12rem');
	const rowHeight = 34;
	const rowGap = 4;
	const minimumVisibleRows = 10;

	function niceStep(value: number) {
		if (value <= 0) return 1;
		const power = Math.pow(10, Math.floor(Math.log10(value)));
		const scaled = value / power;
		if (scaled >= 5) return 5 * power;
		if (scaled >= 2) return 2 * power;
		return power;
	}

	function formatValue(value: number) {
		const abs = Math.abs(value);
		const maximumFractionDigits = abs >= 100 ? 0 : abs >= 10 ? 1 : 2;
		return new Intl.NumberFormat('de-AT', {
			maximumFractionDigits,
			minimumFractionDigits: 0
		}).format(value);
	}

	let extent = $derived.by(() => {
		const values = data.map((item) => Number(item.value ?? 0)).filter(Number.isFinite);
		const rawMin = Math.min(0, ...values);
		const rawMax = Math.max(0, ...values);
		const span = rawMax - rawMin || 1;
		const step = niceStep(span / 4);
		const min = Math.floor(rawMin / step) * step;
		const max = Math.ceil(rawMax / step) * step || step;
		return { min, max, span: max - min || 1, step };
	});

	let ticks = $derived.by(() => {
		const values: number[] = [];
		for (let tick = extent.min; tick <= extent.max + extent.step / 2; tick += extent.step) {
			values.push(Number(tick.toFixed(10)));
		}
		return values.length > 1 ? values : [0, extent.max];
	});

	function xPercent(value: number) {
		return ((value - extent.min) / extent.span) * 100;
	}

	function barLeft(value: number) {
		return `${Math.min(xPercent(0), xPercent(value))}%`;
	}

	function barWidth(value: number) {
		return `${Math.max(0.25, Math.abs(xPercent(value) - xPercent(0)))}%`;
	}

	let zeroPosition = $derived(`${xPercent(0)}%`);
	let rowViewportHeight = $derived(
		`${minimumVisibleRows * rowHeight + (minimumVisibleRows - 1) * rowGap}px`
	);
	let hoveredItem = $derived(hoveredIndex === null ? null : (data[hoveredIndex] ?? null));
	let detailItem = $derived(hoveredItem ?? data[0] ?? null);
	let detailRank = $derived(hoveredIndex === null ? (detailItem ? 1 : null) : hoveredIndex + 1);
</script>

<div
	class="relative flex flex-col overflow-hidden"
	style="--label-column-width: {labelColumnWidth};"
>
	<div
		class="grid shrink-0 gap-3 border-b border-gray-200 bg-white px-4 pt-4 pb-3 shadow-sm md:grid-cols-[minmax(0,1fr)_minmax(16rem,22rem)] md:items-start dark:border-surface-700 dark:bg-surface-800"
	>
		<div class="min-w-0">
			<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">{metricLabel}</h2>
			<p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
				{chartDescription}
			</p>
		</div>

		<div class="min-h-[5.75rem] md:text-right">
			<div
				class="grid min-h-[5.75rem] rounded-lg border border-gray-200 bg-white p-3 text-sm shadow-sm dark:border-surface-700 dark:bg-surface-800"
			>
				{#if detailItem}
					<div class="grid h-full content-between gap-2">
						<div class="flex min-w-0 items-start gap-2 md:justify-end">
							<span class="text-xs font-semibold text-gray-500 tabular-nums dark:text-gray-400">
								#{detailRank}
							</span>
							<span
								class="mt-1 h-3 w-3 shrink-0 rounded-full"
								style="background-color: {detailItem.color}"
							></span>
							<div class="min-w-0">
								<div class="truncate font-semibold text-gray-900 dark:text-gray-50">
									{detailItem.category}
								</div>
								{#if detailItem.party && detailItem.party !== detailItem.category}
									<div class="mt-0.5 truncate text-xs text-gray-500 dark:text-gray-400">
										{detailItem.party}
									</div>
								{/if}
							</div>
						</div>
						<div
							class="flex items-end justify-between gap-4 border-t border-gray-100 pt-2 text-xs md:justify-end dark:border-surface-700 dark:text-gray-200"
						>
							<span class="text-gray-600 dark:text-gray-300">{metricLabel}</span>
							<span
								class="text-lg leading-none font-bold text-gray-900 tabular-nums dark:text-gray-50"
							>
								{formatValue(detailItem.value)}
							</span>
						</div>
					</div>
				{:else if infoQuestion && infoAnswer}
					<div class="group relative inline-block self-start text-left md:text-right">
						<button
							type="button"
							class="rounded-lg border border-primary-300 px-3 py-1.5 text-sm font-semibold hover:bg-primary-100 dark:border-primary-400 dark:hover:bg-surface-700"
						>
							{infoQuestion}
						</button>
						<div
							class="invisible absolute top-10 right-0 z-30 w-80 rounded-xl border border-gray-300 bg-surface-50 p-4 text-left text-sm opacity-0 shadow-lg transition group-hover:visible group-hover:opacity-100 dark:border-surface-600 dark:bg-surface-700"
						>
							<div class="space-y-2 text-gray-700 dark:text-gray-100">
								{@html infoAnswer}
							</div>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<div class="overflow-x-auto">
		<div class="relative min-w-[760px] px-4 pt-4 pb-4">
			<div
				class="chart-scrollbar space-y-1 overflow-y-auto pr-2"
				style="height: {rowViewportHeight};"
				role="list"
				aria-label="Statistikdaten"
			>
				{#each data as item, index}
					<div
						class="chart-row grid items-center gap-3"
						style="min-height: {rowHeight}px;"
						role="listitem"
						tabindex="0"
						onmouseenter={() => (hoveredIndex = index)}
						onmouseleave={() => (hoveredIndex = null)}
						onfocus={() => (hoveredIndex = index)}
						onblur={() => (hoveredIndex = null)}
					>
						<div class="grid min-w-0 grid-cols-[3rem_minmax(0,1fr)] items-center gap-2 pr-1">
							<span
								class="justify-self-end text-[11px] font-semibold text-gray-400 tabular-nums dark:text-gray-500"
							>
								#{index + 1}
							</span>
							<div class="flex min-w-0 items-center gap-2">
								<span class="h-3 w-3 shrink-0 rounded-full" style="background-color: {item.color}"
								></span>
								<span
									class="min-w-0 truncate text-right text-xs font-semibold text-gray-800 dark:text-gray-100"
									title={item.category}
								>
									{item.category}
								</span>
							</div>
						</div>

						<div class="relative h-7 min-w-0">
							{#each ticks as tick}
								<div
									class="absolute top-0 bottom-0 w-px bg-gray-200 dark:bg-surface-700"
									style="left: {xPercent(tick)}%;"
								></div>
							{/each}
							<div
								class="absolute top-0 bottom-0 w-px bg-gray-400 dark:bg-surface-500"
								style="left: {zeroPosition};"
							></div>
							<div
								class="absolute top-1/2 h-4 -translate-y-1/2 rounded-sm transition-all"
								class:opacity-70={hoveredIndex !== null && hoveredIndex !== index}
								class:brightness-110={hoveredIndex === index}
								style="left: {barLeft(item.value)}; width: {barWidth(
									item.value
								)}; background-color: {item.color};"
								role="img"
								aria-label="Rang {index + 1}, {item.category}: {metricLabel} {formatValue(
									item.value
								)}"
							></div>
							<span
								class="absolute top-1/2 -translate-y-1/2 px-2 text-xs font-semibold text-gray-700 tabular-nums dark:text-gray-200"
								class:hidden={Math.abs(item.value) < extent.span * 0.02}
								style="left: calc({xPercent(item.value)}% + {item.value >= 0
									? '4px'
									: '-4.25rem'});"
							>
								{formatValue(item.value)}
							</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<div
		class="shrink-0 overflow-x-auto border-t border-gray-200 bg-white dark:border-surface-700 dark:bg-surface-800"
	>
		<div class="chart-axis grid min-w-[760px] gap-3 px-4 pt-2 pb-3">
			<div
				class="text-right text-xs font-semibold text-gray-600 dark:text-gray-300"
				style="grid-column: 1;"
			>
				{metricLabel}
			</div>
			<div class="relative h-8" style="grid-column: 2;">
				<div
					class="absolute top-0 h-2 w-px bg-gray-500 dark:bg-surface-400"
					style="left: {zeroPosition};"
				></div>
				<div class="absolute top-1 right-0 left-0 h-px bg-gray-300 dark:bg-surface-600"></div>
				{#each ticks as tick}
					<div class="absolute top-0 -translate-x-1/2" style="left: {xPercent(tick)}%;">
						<div class="mx-auto h-2 w-px bg-gray-400 dark:bg-surface-500"></div>
						<div
							class="mt-1 text-center text-[11px] font-semibold whitespace-nowrap text-gray-600 tabular-nums dark:text-gray-300"
						>
							{formatValue(tick)}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.chart-row,
	.chart-axis {
		grid-template-columns: var(--label-column-width) minmax(24rem, 1fr);
	}

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
