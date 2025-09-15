<script lang="ts">
	import type { Bubble } from '$lib/parliament';
	import { onMount, onDestroy } from 'svelte';

	export let circles2d: Bubble[][] = [];
	export let width = 830;
	export let height = 900;
	let clazz = '';
	export { clazz as class };

	let canvas: HTMLCanvasElement;
	let container: HTMLDivElement;
	let ctx: CanvasRenderingContext2D | null = null;

	$: flat = circles2d ? circles2d.flat(1) : [];

	const logicalW = width;
	const logicalH = height * 0.5 + 60;

	let resizeObserver: ResizeObserver | null = null;
	let raf = 0;
	let offsetX = 0;
	let offsetY = 0;
	let pixelRatio = 1;

	function scheduleResize() {
		if (raf) cancelAnimationFrame(raf);
		raf = requestAnimationFrame(() => {
			resizeAndRedraw();
			raf = 0;
		});
	}

	function resizeAndRedraw() {
		if (!canvas || !container) return;
		const rect = container.getBoundingClientRect();
		const containerW = Math.max(1, rect.width);
		pixelRatio = Math.max(1, window.devicePixelRatio || 1);

		const scale = containerW / logicalW;
		// const scaleY = containerH / logicalH;
		// scale = Math.min(scaleX, scaleY);

		const displayW = logicalW * scale;
		const displayH = logicalH * scale;

		canvas.style.width = `${containerW}px`;
		canvas.style.height = `${displayH}px`;

		const internalW = Math.round(containerW * pixelRatio);
		const internalH = Math.round(displayH * pixelRatio);

		if (canvas.width !== internalW || canvas.height !== internalH) {
			canvas.width = internalW;
			canvas.height = internalH;
			ctx = canvas.getContext('2d');
		}
		if (!ctx) return;

		offsetX = (containerW - displayW) / 2;
		offsetY = (displayH - logicalH * scale) / 2;

		ctx.setTransform(
			pixelRatio * scale,
			0,
			0,
			pixelRatio * scale,
			Math.round(offsetX * pixelRatio),
			Math.round(offsetY * pixelRatio)
		);
		drawAll();
	}

	function drawAll() {
		if (!ctx || !canvas) return;
		ctx.save();
		ctx.setTransform(1, 0, 0, 1, 0, 0);
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.restore();

		for (let i = 0; i < flat.length; i++) {
			const c = flat[i];
			ctx.beginPath();
			ctx.arc(c.x, c.y, c.r, 0, Math.PI * 2);
			ctx.globalAlpha = c.opacity ?? 1;
			if (c.color) ctx.fillStyle = c.color;
			ctx.fill();
		}
		ctx.globalAlpha = 1;
	}

	onMount(() => {
		ctx = canvas.getContext('2d');
		resizeObserver = new ResizeObserver(scheduleResize);
		resizeObserver.observe(container);
		window.addEventListener('resize', scheduleResize);
		scheduleResize();
	});

	onDestroy(() => {
		if (resizeObserver) resizeObserver.disconnect();
		window.removeEventListener('resize', scheduleResize);
		if (raf) cancelAnimationFrame(raf);
		ctx = null;
	});

	$: if (flat) scheduleResize();
</script>

<div class={clazz} bind:this={container}>
	<canvas bind:this={canvas}></canvas>
</div>

<style>
	canvas {
		display: block;
	}
</style>
