<script lang="ts">
	import { Tooltip } from 'bits-ui';
	import type { Vote } from '$lib/types';
	import { localeStore } from '$lib/i18n/i18n.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { VOTE_COLORS } from '$lib/voteColors';

	let {
		vote,
		size = 20
	}: {
		vote: Vote;
		size?: number;
	} = $props();

	const VIEWBOX = 20;
	const CENTER = VIEWBOX / 2;
	const RADIUS = 8.5;
	const STROKE_WIDTH = 3;
	const CIRCUMFERENCE = 2 * Math.PI * RADIUS;
	/** gap between two segments in the circle */
	const GAP = 1.2;

	const donutData = $derived.by(() => {
		// Read localeStore.value to create a reactive dependency
		const _ = localeStore.value;
		return [
			{ category: t('donut.inFavor'), value: vote.infavor_count, color: VOTE_COLORS.infavor },
			{ category: t('donut.against'), value: vote.against_count, color: VOTE_COLORS.against },
			{
				category: t('donut.abstention'),
				value: vote.abstention_count,
				color: VOTE_COLORS.abstention
			},
			{ category: t('donut.absent'), value: vote.absence_count, color: VOTE_COLORS.absent }
		].filter((d) => d.value > 0);
	});

	const segments = $derived.by(() => {
		const total = donutData.reduce((sum, d) => sum + d.value, 0);
		if (total === 0) {
			return [];
		}
		const gap = donutData.length > 1 ? GAP : 0;
		let start = 0;
		return donutData.map((d) => {
			const length = (d.value / total) * CIRCUMFERENCE;
			const drawn = Math.max(length - gap, 0.01);
			const segment = {
				...d,
				dasharray: `${drawn} ${CIRCUMFERENCE - drawn}`,
				dashoffset: -start
			};
			start += length;
			return segment;
		});
	});
</script>

<Tooltip.Provider
	delayDuration={1}
	disableCloseOnTriggerClick={true}
	disableHoverableContent={true}
>
	<Tooltip.Root>
		<Tooltip.Trigger
			class="block shrink-0 cursor-default text-blue-500"
			style="width:{size}px; height:{size}px;"
			aria-label={t('donut.ariaLabel', { party: vote.party })}
		>
			<svg
				viewBox="0 0 {VIEWBOX} {VIEWBOX}"
				width={size}
				height={size}
				class="block"
				aria-hidden="true"
			>
				<!-- rotate: Segmente starten oben statt rechts -->
				<g transform="rotate(-90 {CENTER} {CENTER})">
					{#each segments as segment (segment.category)}
						<circle
							cx={CENTER}
							cy={CENTER}
							r={RADIUS}
							fill="none"
							stroke={segment.color}
							stroke-width={STROKE_WIDTH}
							stroke-dasharray={segment.dasharray}
							stroke-dashoffset={segment.dashoffset}
						/>
					{/each}
				</g>
			</svg>
		</Tooltip.Trigger>

		<Tooltip.Content
			sideOffset={6}
			class="pointer-events-none z-[100] w-40 rounded-lg bg-white p-2 text-xs shadow-lg dark:bg-surface-700"
		>
			<Tooltip.Arrow class="text-white dark:text-surface-700" />
			<div class="mb-1.5 font-semibold text-gray-800 dark:text-gray-100">{vote.party}</div>
			<div class="flex flex-col gap-1">
				{#each donutData as d (d.category)}
					<div class="flex items-center justify-between gap-2">
						<div class="flex items-center gap-1.5">
							<div class="h-2 w-2 rounded-full" style="background-color: {d.color};"></div>
							<span class="text-gray-700 dark:text-gray-200">{d.category}</span>
						</div>
						<span class="text-gray-800 tabular-nums dark:text-gray-100">{d.value}</span>
					</div>
				{/each}
			</div>
		</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
