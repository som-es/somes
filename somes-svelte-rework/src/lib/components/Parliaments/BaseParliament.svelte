<script lang="ts">
	import type { Bubble } from '$lib/parliament';

	export let circles2d: Bubble[][];
	export let selected: Bubble | null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

	export let width = 830;
	export let height = 900;

	console.log(circles2d);

	let clazz = '';
	export { clazz as class };
</script>

<div class={clazz}>
	<svg viewBox="0 0 {width} {height * 0.5 + 60}" style="max-width: 100%;">
		{#each circles2d.flat(1) as circle, i}
			<circle
				class="translated-circle outline-none"
				type="button"
				cx={circle.x}
				cy={circle.y}
				r={circle.r}
				role="button"
				on:click={(event) => {
					if (preview) return;
					select(circle, event);
				}}
				on:keypress={(event) => {
					if (preview) return;
					select(circle, event);
				}}
				fill={circle.color}
				fill-opacity={circle.opacity}
				tabindex={100 + i}
				stroke={circle == selected ? 'orange' : ''}
				stroke-width={circle == selected ? '4' : ''}
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
