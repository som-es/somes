<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare, StanceTopicScore } from '$lib/types';
	import { Dialog } from 'bits-ui';

	interface Props {
		stances: StanceTopicScore[];
	}

	let { stances }: Props = $props();

	const DOT_R = 6;
	const CHART_W = 700;
	const PAD_LEFT = 10;
	const PAD_RIGHT = 10;
	const PAD_TOP = 10;
	const ROW_H = 30;
	const BOTTOM_H = 30;

	function scoreToX(score: number): number {
		return PAD_LEFT + ((score * 2 + 1) / 2) * (CHART_W - PAD_LEFT - PAD_RIGHT);
	}

	const positioned = [...stances].sort((a, b) => a.score - b.score).map((item, i) => ({ ...item, x: scoreToX(item.score), row: i }));

	const numRows = positioned.length > 0 ? Math.max(...positioned.map((p) => p.row)) + 1 : 1;
	const CHART_H = numRows * ROW_H + PAD_TOP + BOTTOM_H;

	const axisY = CHART_H - BOTTOM_H;
	const centerX = PAD_LEFT + (CHART_W - PAD_LEFT - PAD_RIGHT) / 2;

	function rowToY(row: number): number {
		return PAD_TOP + row * ROW_H + DOT_R;
	}
</script>

<div class="card p-8">
	<div class="flex justify-between">
		<span class="font-bold text-2xl">Richtung</span>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<span class="text-lg">Politische Ausrichtung nach Thema</span>

	<div class="mt-4 w-full overflow-x-auto rounded-sm p-4">
		<svg viewBox="0 0 {CHART_W} {CHART_H}" class="w-full" style="height: {CHART_H}px;">
			<line x1={centerX} y1={PAD_TOP} x2={centerX} y2={axisY} stroke="#888" stroke-width="1" />

			<line
				x1={PAD_LEFT}
				y1={axisY}
				x2={CHART_W - PAD_RIGHT}
				y2={axisY}
				stroke="#888"
				stroke-width="1"
			/>

			{#each positioned as item}
				{@const y = rowToY(item.row)}
				<circle cx={item.x} cy={y} r={DOT_R} fill={topicColors.get(item.topic) ?? '#888'} />
				<text x={item.x + DOT_R + 4} y={y + 5} font-size="12" fill="currentColor"
					>{item.topic}</text
				>
			{/each}

			<text x={PAD_LEFT} y={axisY + 20} font-size="13" fill="#888" text-anchor="start">← Links</text>
			<text x={centerX} y={axisY + 20} font-size="13" fill="#888" text-anchor="middle">Mitte</text>
			<text x={CHART_W - PAD_RIGHT} y={axisY + 20} font-size="13" fill="#888" text-anchor="end"
				>Rechts →</text
			>
		</svg>
	</div>
</div>
