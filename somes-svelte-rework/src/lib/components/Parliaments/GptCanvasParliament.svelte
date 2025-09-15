<script lang="ts">
	import type { Bubble } from '$lib/parliament';
	import { onMount, onDestroy } from 'svelte';

	export let circles2d: Bubble[][] = [];

	let clazz = '';
	export { clazz as class };
	export let width = 830;
	export let height = 900;

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null = null;

	$: flat = circles2d ? circles2d.flat(1) : [];

	function drawAll() {
		if (!ctx) return;
		const w = canvas.width;
		const h = canvas.height;
		ctx.clearRect(0, 0, w, h);
		for (let i = 0; i < flat.length; i++) {
			const c = flat[i];
			ctx.beginPath();
			ctx.arc(c.x, c.y, c.r, 0, Math.PI * 2);
			ctx.globalAlpha = c.opacity;
			if (c.color) {
				ctx.fillStyle = c.color;
			}
			ctx.fill();
		}
	}

	$: drawAll();

	onMount(() => {
		ctx = canvas.getContext('2d');
		const pixelRatio = Math.max(1, window.devicePixelRatio || 1);
		canvas.width = width * pixelRatio;
		canvas.height = (height * 0.5 + 60) * pixelRatio;
		canvas.style.width = `${width}px`;
		canvas.style.height = `${height * 0.5 + 60}px`;
		if (ctx) ctx.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
		drawAll();
	});

	onDestroy(() => {
		ctx = null;
	});
</script>

<div class={clazz}>
	<canvas bind:this={canvas} style="width:100%;"></canvas>
</div>
