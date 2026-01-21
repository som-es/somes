<script lang="ts">
	interface Props {
		stops?: any;
		isLightMode?: boolean;
		mark50?: boolean;
		mark66?: boolean;
	}

	let {
		stops = [{ color: 'red', start: 0, end: 360 }],
		isLightMode = true,
		mark50 = false,
		mark66 = false
	}: Props = $props();

	function setColorValue(color: any): string {
		return color;
	}
	let cone: string | undefined = $derived.by(() => {
		const cssStops = stops.map((v: any) => `${setColorValue(v.color)} ${v.start}deg ${v.end}deg`);
		const conicGradient = `conic-gradient(${cssStops.join(', ')})`;
		return conicGradient;
	});

</script>

{#if cone}
<div class="wrapper">

	<div class="donut {isLightMode ? '' : 'dark-donut'}" style:background={cone}></div>

	{#if mark50}
		<div class="marker mark-50"></div>
	{/if}
	{#if mark66}
		<div class="marker-66"></div>
	{/if}
</div>
{/if}

<style>
	.wrapper {
		position: relative;
		width: 60px;
		height: 60px;
	}
	.donut::before {
		content: '';
		min-height: 40px;
		min-width: 40px;
		width: 40px;
		height: 40px;
		border-radius: 50%;
		/* background: green; */
		background: var(--color-primary-300);
		z-index: 100;
	}

	.dark-donut::before {
		background: var(--color-primary-500);
	}

	.donut {
		min-height: 60px;
		min-width: 60px;
		width: 60px;
		height: 60px;
		border-radius: 50%;

		display: flex;
		align-items: center;
		justify-content: center;
		position: absolute;
		top: 0;
		left: 0;
	}
	.marker {
		position: absolute;
		top: 50%;
		left: 46%;
		width: 3px;           /* thickness of marker */
		height: 30%;           /* radius length */
		background: black;
		transform-origin: bottom center;
		z-index: 100;
	}

	.mark-50 {
		transform: rotate(180deg) translateY(10%);
	}
	
	.marker-66 {
		position: absolute;
		top: 19%;
		left: -10%;
		width: 3px;           /* thickness of marker */
		height: 30%;           /* radius length */
		background: black;
		transform-origin: bottom center;
		z-index: 100;
		transform: rotate(90deg) translateY(0%);
	}
	
</style>
