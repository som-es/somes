<script lang="ts">
	import type { Bubble } from '$lib/parliament';

	export let circles2d: Bubble[][] = [];

	export let selected: Bubble | null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

	export const width = 830;
	export const height = 900;
	const MAX_WIDTH: number = 10;

	let clazz = '';
	export { clazz as class };

	$: circles2d;
	// console.log(circles2d);
</script>

<div class={clazz}>
	<svg viewBox="0 0 {width} {height * 0.5 + 60}" style="max-width: 100%;">
		{#each circles2d as circleRow, r}
			{#each circleRow as circle, c}
				<circle
					class="outline-hidden"
					type="button"
					cx={((c % MAX_WIDTH) + r * MAX_WIDTH) * 30}
					cy={Math.floor(c / MAX_WIDTH) * 30 + 30}
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
					tabindex={100 + r * circleRow.length + c}
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
		{/each}
	</svg>
</div>
