<script lang="ts">
	import { browser } from '$app/environment';
	import type { Bubble } from '$lib/parliament';
	import { onDestroy, onMount } from 'svelte';

	let active = $state(false);

	interface Props {
		circles2d: Bubble[][];
		selected?: Bubble | null;
		preview?: boolean;
		select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;
		forceColor?: string | null;
		width?: number;
		height?: number;
		class?: string;
	}

	let {
		circles2d,
		selected = null,
		preview = false,
		select,
		forceColor = null,
		width = 830,
		height = 900,
		class: clazz = ''
	}: Props = $props();
	
	function handleKey(e: KeyboardEvent) {
		if (!active || preview || !circles2d.length) return;
		if (!selected || !selected.del || !selected.del.seat_row || !selected.del.seat_col) return;

		let next: Bubble | undefined;
		if (e.key === 'ArrowRight') {
			next = circles2d[selected.del.seat_row - 1][selected.del.seat_col];
			if (!next || !next.del)
				next = circles2d[selected.del.seat_row - 1][selected.del.seat_col + 1];
		} else if (e.key === 'ArrowLeft') {
			next = circles2d[selected.del.seat_row - 1][selected.del.seat_col - 2];
			if (!next || !next.del)
				next = circles2d[selected.del.seat_row - 1][selected.del.seat_col - 3];
		} else if (e.key === 'ArrowUp') {
			next = circles2d[selected.del.seat_row][selected.del.seat_col - 1];
		} else if (e.key === 'ArrowDown') {
			next = circles2d[selected.del.seat_row - 2][selected.del.seat_col - 1];
		}

		if (next) {
			select(next, e);
			e.preventDefault();
		}
	}

	function deactivateOnOutsideClick(e: MouseEvent) {
		const svg = document.querySelector('.parliament-svg');
		if (svg && !svg.contains(e.target as Node)) {
			active = false;
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKey);
		window.addEventListener('mousedown', deactivateOnOutsideClick);
	});

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleKey);
			window.removeEventListener('mousedown', deactivateOnOutsideClick);
		}
	});
</script>

<div class={clazz}>
	<svg
		viewBox="0 0 {width}
		{height * 0.5 + 60}"
		style="width: 100%;"
		class="parliament-svg hover:cursor-default"
		onclick={() => (active = true)}
		onkeydown={() => (active = true)}
		role="button"
		tabindex="0"
	>
		{#each circles2d.flat(1) as circle, i}
			<circle
				class="translated-circle outline-hidden"
				type="button"
				cx={circle.x}
				cy={circle.y}
				r={circle.r}
				role="button"
				onclick={(event) => {
					if (preview) return;
					select(circle, event);
				}}
				onkeypress={(event) => {
					if (preview) return;
					select(circle, event);
				}}
				fill={forceColor ? forceColor : circle.color}
				fill-opacity={circle.opacity}
				tabindex={0}
				stroke={circle.del != null && circle.del?.id == selected?.del?.id ? 'orange' : ''}
				stroke-width={circle.del != null && circle.del?.id == selected?.del?.id ? '4' : ''}
			>
				{#if circle.title !== null && circle.del}
					<title>
						{circle.del.name}: {circle.title}
					</title>
				{/if}
			</circle>
		{/each}
	</svg>
</div>

<style>
</style>
