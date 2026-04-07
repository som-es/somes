<script lang="ts">
	import type { DataPoint } from './types';

	let {
		width = 240,
		height = 240,
		xLabels,
		yLabels,
		dataPoints
	}: {
		width?: number;
		height?: number;
		xLabels: string[];
		yLabels: string[];
		dataPoints: { [key: string]: DataPoint[] };
	} = $props();

	let canvas: HTMLCanvasElement | undefined = $state();

	// takes with from outer div and calculates svg dimensions
	let containerWidth = $state(0);

	const drawChart = (textColor: string, lineColor: string) => {
		if (!canvas || containerWidth === 0) return;
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		const scale = containerWidth / width;
		const canvasW = containerWidth;
		const canvasH = Math.round(height * scale);

		canvas.width = canvasW;
		canvas.height = canvasH;

		ctx.clearRect(0, 0, canvasW, canvasH);

		ctx.strokeStyle = lineColor;
		ctx.lineWidth = 2;
		ctx.beginPath();
		ctx.moveTo(canvasW / 2, 0);
		ctx.lineTo(canvasW / 2, canvasH);
		ctx.moveTo(0, canvasH / 2);
		ctx.lineTo(canvasW, canvasH / 2);
		ctx.stroke();

		const fontSize = Math.max(10, Math.round(14 * scale));
		const labelOffset = Math.max(12, Math.round(20 * scale));

		ctx.fillStyle = textColor;
		ctx.font = `${fontSize}px Arial`;
		ctx.textAlign = 'center';
		ctx.textBaseline = 'middle';

		ctx.save();
		ctx.translate(canvasW - labelOffset, canvasH / 2);
		ctx.rotate(-Math.PI / 2);
		ctx.fillText(xLabels[0], 0, 0);
		ctx.restore();

		ctx.save();
		ctx.translate(labelOffset, canvasH / 2);
		ctx.rotate(-Math.PI / 2);
		ctx.fillText(xLabels[1], 0, 0);
		ctx.restore();

		ctx.fillText(yLabels[0], canvasW / 2, labelOffset);
		ctx.fillText(yLabels[1], canvasW / 2, canvasH - labelOffset);

		const dotRadius = Math.max(4, Math.round(8 * scale));
		const pointFontSize = Math.max(9, Math.round(11 * scale));

		Object.values(dataPoints).forEach((quadrant) => {
			quadrant.forEach(({ x, y, label, color }) => {
				const canvasX = canvasW / 2 + x * scale;
				const canvasY = canvasH / 2 - y * scale;

				ctx.fillStyle = color;
				ctx.beginPath();
				ctx.arc(canvasX, canvasY, dotRadius, 0, Math.PI * 2);
				ctx.fill();

				ctx.fillStyle = textColor;
				ctx.font = `${pointFontSize}px Arial`;
				ctx.textAlign = 'center';
				ctx.fillText(label, canvasX, canvasY - dotRadius - 4);
			});
		});
	};

	const drawWithMode = () => {
		// TODO: fix white mode
		const isLightMode = true;
		const borderColor = isLightMode ? '#0000002b' : '#FFFFFF2b';
		const textColor = isLightMode ? '#000000' : '#FFFFFF';
		drawChart(textColor, borderColor);
	};

	$effect(() => {
		containerWidth;
		dataPoints;
		drawWithMode();
	});
</script>

<div class="w-full" bind:clientWidth={containerWidth}>
	<canvas class="border-black dark:border-white" bind:this={canvas}></canvas>
</div>

<style>
	canvas {
		border: 3px solid;
		display: block;
		width: 100%;
	}
</style>
