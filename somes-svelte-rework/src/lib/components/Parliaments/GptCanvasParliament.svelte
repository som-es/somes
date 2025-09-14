<script lang="ts">
	import type { Bubble } from '$lib/parliament';
	import { onMount, onDestroy } from 'svelte';

	export let circles2d: Bubble[][] = [];
	export let selected: Bubble | null = null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

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
			ctx.fillStyle = c.color;
			ctx.fill();
			if (c.del?.id === selected?.del?.id) {
				ctx.lineWidth = 4;
				ctx.strokeStyle = 'orange';
				ctx.stroke();
			}
		}
	}

	$: drawAll();

	function getPointerPos(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		return { x: e.clientX - rect.left, y: e.clientY - rect.top };
	}

	function handleClick(e: MouseEvent) {
		if (preview) return;
		const p = getPointerPos(e);
		for (let i = flat.length - 1; i >= 0; i--) {
			const c = flat[i];
			const dx = p.x - c.x;
			const dy = p.y - c.y;
			if (dx * dx + dy * dy <= c.r * c.r) {
				select(c, e);
				break;
			}
		}
	}

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

<canvas bind:this={canvas} on:click={handleClick} style="width:100%; height:auto;"></canvas>
