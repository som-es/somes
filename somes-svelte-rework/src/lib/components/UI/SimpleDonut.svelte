<script lang="ts">
	import { afterUpdate } from 'svelte';

	import type { ConicStop } from '@skeletonlabs/skeleton';

	export let stops: ConicStop[] = [{ color: 'red', start: 0, end: 360 }];
	export let isLightMode: boolean = true;

	function setColorValue(color: any): string {
		return color;
	}
	let cone: string;

	function genConicGradient(): void {
		let d: any = stops.map((v) => `${setColorValue(v.color)} ${v.start}deg ${v.end}deg`);
		cone = `conic-gradient(${d.join(', ')})`;
	}
	afterUpdate(() => {
		genConicGradient();
	});
</script>

{#if cone}
	<div class="donut {isLightMode ? '' : 'dark-donut'}" style:background={cone}></div>
{/if}

<style>
	.donut::before {
		content: '';
		min-height: 40px;
		min-width: 40px;
		width: 40px;
		height: 40px;
		border-radius: 50%;
		/* background: rgb(var(--bg-primary-300)); */
		background: rgb(var(--color-primary-300));
		z-index: 10;
	}

	.dark-donut::before {
		background: rgb(var(--color-primary-500));
	}

	/* @media (prefers-color-scheme: dark) {
        .donut::before {
            background: rgb(var(--color-primary-500));
        }
    } */

	.donut {
		min-height: 60px;
		min-width: 60px;
		width: 60px;
		height: 60px;
		border-radius: 50%;

		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
