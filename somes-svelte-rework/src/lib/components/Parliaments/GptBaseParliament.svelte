<script lang="ts">
	import type { Bubble } from '$lib/parliament';
	import { onMount } from 'svelte';

	export let circles2d: Bubble[][] = [];
	export let selected: Bubble | null = null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

	export let width = 830;
	export let height = 900;

	let clazz = '';
	export { clazz as class };

	$: flat = circles2d ? circles2d.flat(1) : [];
	$: idMap = new Map(flat.map((b) => [String(b.id), b]));

	function handleClick(e: MouseEvent) {
		if (preview) return;
		const target = e.target as SVGElement;
		const id = target?.dataset?.id;
		if (!id) return;
		const bubble = idMap.get(id);
		if (bubble) select(bubble, e);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (preview) return;
		const k = e.key;
		if (k !== 'Enter' && k !== ' ') return;
		const target = e.target as SVGElement;
		const id = target?.dataset?.id;
		if (!id) return;
		const bubble = idMap.get(id);
		if (bubble) {
			e.preventDefault();
			select(bubble, e);
		}
	}
</script>

<div class={clazz}>
	<svg
		viewBox="0 0 {width} {height * 0.5 + 60}"
		style="width: 100%;"
		on:click={handleClick}
		on:keydown={handleKeydown}
	>
		{#each flat as circle (circle.id)}
			<circle
				class="translated-circle outline-none"
				class:selected={circle.del?.id === selected?.del?.id}
				cx={circle.x}
				cy={circle.y}
				r={circle.r}
				role="button"
				tabindex={0}
				data-id={circle.id}
				fill={circle.color}
				fill-opacity={circle.opacity}
			>
				{#if circle.title !== null && circle.del}
					<title>{circle.del.name}: {circle.title}</title>
				{/if}
			</circle>
		{/each}
	</svg>
</div>

<style>
	.translated-circle {
		transition: transform 120ms;
	}
	.translated-circle:focus {
		outline: none;
	}
	.selected {
		stroke: orange;
		stroke-width: 4;
	}
</style>
