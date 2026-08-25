<script lang="ts">
	import { t } from '$lib/i18n';
	import { topicColors } from '$lib/interestColors';
	import type { StanceTopicScore } from '$lib/types';

	interface Props {
		stances: StanceTopicScore[];
	}

	let { stances }: Props = $props();

	let windowWidth = $state(window?.innerWidth ?? 700);
	let isMobile = $derived(windowWidth < 640);

	const CHART_W = 700;
	const PAD_LEFT = 10;
	const PAD_RIGHT = 10;
	const PAD_TOP = 10;

	const DOT_R = $derived(isMobile ? 12 : 4);
	const ROW_H = $derived(isMobile ? 75 : 26);
	const BOTTOM_H = $derived(isMobile ? 55 : 30);
	const FONT_TOPIC = $derived(isMobile ? 26 : 10);
	const FONT_AXIS = $derived(isMobile ? 26 : 11);
	const Y_AXIS_TEXT = $derived(isMobile ? 30 : 20);
	const LABEL_OFFSET = $derived(isMobile ? 8 : 4);

	// Maps score (-0.5..0.5) to SVG x coordinate
	function scoreToX(score: number): number {
		return PAD_LEFT + ((score * 2 + 1) / 2) * (CHART_W - PAD_LEFT - PAD_RIGHT);
	}

	// Keep only the 8 topics with the strongest stance (highest absolute score),
	// then sort them left (most negative) to right
	const positioned = (() => {
		return stances
			.sort((a, b) => a.score - b.score)
			.map((item, i) => ({ ...item, x: scoreToX(item.score), row: i }));
	})();

	const numRows = positioned.length > 0 ? Math.max(...positioned.map((p) => p.row)) + 1 : 1;
	const centerX = PAD_LEFT + (CHART_W - PAD_LEFT - PAD_RIGHT) / 2;

	const CHART_H = $derived(numRows * ROW_H + PAD_TOP + BOTTOM_H);
	const axisY = $derived(CHART_H - BOTTOM_H);

	function rowToY(row: number): number {
		return PAD_TOP + row * ROW_H + DOT_R;
	}
</script>

<svelte:window bind:innerWidth={windowWidth} />

<svg viewBox="0 0 {CHART_W} {CHART_H}" class="w-full">
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
		{@const isRight = item.x > centerX}
		<circle cx={item.x} cy={y} r={DOT_R} fill={topicColors.get(item.topic) ?? '#888'} />
		<text
			x={isRight ? item.x - DOT_R - LABEL_OFFSET : item.x + DOT_R + LABEL_OFFSET}
			y={y + (isMobile ? 8 : 3)}
			font-size={FONT_TOPIC}
			fill="currentColor"
			text-anchor={isRight ? 'end' : 'start'}>{item.topic}</text
		>
	{/each}

	<!-- Axis labels -->
	<text x={PAD_LEFT} y={axisY + Y_AXIS_TEXT} font-size={FONT_AXIS} fill="#888" text-anchor="start"
		>← {t("spectrum.stance.label.left")}</text
	>
	<text x={centerX} y={axisY + Y_AXIS_TEXT} font-size={FONT_AXIS} fill="#888" text-anchor="middle"
		>{t("spectrum.stance.label.center")}</text
	>
	<text
		x={CHART_W - PAD_RIGHT}
		y={axisY + Y_AXIS_TEXT}
		font-size={FONT_AXIS}
		fill="#888"
		text-anchor="end">{t("spectrum.stance.label.right")} →</text
	>
</svg>
