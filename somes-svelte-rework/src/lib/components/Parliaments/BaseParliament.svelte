<script lang="ts">
	import type { Bubble } from '$lib/parliament';

	export let circles2d: Bubble[][];
	export let selected: Bubble;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

	export let width = 830;
	export let height = 900;
</script>

<div
	style="pointer-events: {preview ? 'none' : 'auto'}; keyboard-events: {preview ? 'none' : 'auto'}"
>
	<svg viewBox="0 0 {width} {height * 0.5 + 60}" style="max-width: 100%;">
		{#each circles2d.flat(1) as circle, i}
			<circle
				class="translated-circle outline-none"
				type="button"
				cx={circle.x}
				cy={circle.y}
				r={circle.r}
				role="button"
				on:click={(event) => select(circle, event)}
				on:keypress={(event) => select(circle, event)}
				fill={circle.color}
				fill-opacity={circle.opacity}
				tabindex={100 + i}
				stroke={circle == selected ? 'orange' : ''}
				stroke-width={circle == selected ? '4' : ''}
			/>
		{/each}
	</svg>
</div>

<style>
</style>
