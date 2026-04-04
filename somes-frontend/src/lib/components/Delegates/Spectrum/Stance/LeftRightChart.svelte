<script lang="ts">
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare, StanceTopicScore } from '$lib/types';
	import ExtendInfoDialog from '../../ExtendInfoDialog.svelte';
	import DetailedLeftRightChart from './DetailedLeftRightChart.svelte';

	
	interface Props {
		stances: StanceTopicScore[];
		interests: InterestShare[];
	}

	let { stances, interests }: Props = $props();

	const DOT_R = 6;
	const CHART_W = 700;
	const PAD_LEFT = 10;
	const PAD_RIGHT = 10;
	const PAD_TOP = 10;
	const ROW_H = 30;
	const BOTTOM_H = 30;

	// Maps score (-0.5..0.5) to SVG x coordinate
	function scoreToX(score: number): number {
		return PAD_LEFT + ((score * 2 + 1) / 2) * (CHART_W - PAD_LEFT - PAD_RIGHT);
	}

	// Keep only the 8 topics with the strongest stance (highest absolute score),
	// then sort them left (most negative) to right
	const positioned = (() => {
		let tempStance: (StanceTopicScore & {occ: number})[] = [];
		if(interests.length > 0){
			stances.map((s) => {
				const intr = interests.find((i) => i.topic === s.topic);
				tempStance.push({ ...s, occ: intr?.occurences ?? 0 });
			});
		}else{
			stances.map((s) => {
				tempStance.push({ ...s, occ: 0 });
			});
		}
		return tempStance
		.sort((a, b) => b.occ - a.occ)
		.slice(0,8)
		.sort((a, b) => a.score - b.score)
		.map((item, i) => ({ ...item, x: scoreToX(item.score), row: i }));
	})()
		

	const numRows = positioned.length > 0 ? Math.max(...positioned.map((p) => p.row)) + 1 : 1;
	const CHART_H = numRows * ROW_H + PAD_TOP + BOTTOM_H;

	const axisY = CHART_H - BOTTOM_H;
	const centerX = PAD_LEFT + (CHART_W - PAD_LEFT - PAD_RIGHT) / 2;

	function rowToY(row: number): number {
		return PAD_TOP + row * ROW_H + DOT_R;
	}
</script>

<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
	<div class="w-full flex items-center justify-between">
		<div class="flex flex-col">
			<span class="text-lg font-bold text-black xl:text-xl dark:text-white">
				Richtung
			</span>
		</div>

		<div class="flex items-center gap-2">
			{#if stances.length > 8}
				<ExtendInfoDialog title="Details">
					<DetailedLeftRightChart {stances} />
				</ExtendInfoDialog>
			{/if}
		</div>
	</div>

	<svg viewBox="0 0 {CHART_W} {CHART_H}" class="w-full" style="height: {CHART_H}px;">
		<!-- Center vertical line -->
		<line x1={centerX} y1={PAD_TOP} x2={centerX} y2={axisY} stroke="#888" stroke-width="1" />

		<!-- Horizontal axis line -->
		<line
			x1={PAD_LEFT}
			y1={axisY}
			x2={CHART_W - PAD_RIGHT}
			y2={axisY}
			stroke="#888"
			stroke-width="1"
		/>

		<!-- Dots and labels -->
		{#each positioned as item}
			{@const y = rowToY(item.row)}
			<circle cx={item.x} cy={y} r={DOT_R} fill={topicColors.get(item.topic) ?? '#888'} />
			<text x={item.x + DOT_R + 4} y={y + 5} font-size="12" fill="currentColor"
				>{item.topic}</text
			>
		{/each}

		<!-- Axis labels -->
		<text x={PAD_LEFT} y={axisY + 20} font-size="13" fill="#888" text-anchor="start">← Links</text>
		<text x={centerX} y={axisY + 20} font-size="13" fill="#888" text-anchor="middle">Mitte</text>
		<text x={CHART_W - PAD_RIGHT} y={axisY + 20} font-size="13" fill="#888" text-anchor="end"
			>Rechts →</text
		>
	</svg>
</div>
