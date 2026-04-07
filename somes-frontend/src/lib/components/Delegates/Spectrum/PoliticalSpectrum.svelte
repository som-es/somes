<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, PoliticalPosition } from '$lib/types';
	import QuadrantChart from '../../GeneralCharts/QuadrantChart.svelte';
	import type { DataPoint } from '../../GeneralCharts/types';



	let { politicalPosition, delegate }: { politicalPosition: PoliticalPosition; delegate: Delegate } = $props();

	let windowWidth = $state(700);
	let isMobile = $derived(windowWidth < 640);

	let color = $derived(partyToColor(delegate.party));

	let leftScore = $derived(politicalPosition.is_not_left - politicalPosition.is_left);
	let liberalScore = $derived(politicalPosition.is_not_liberal - politicalPosition.is_liberal);

	const SCALAR = 20;
	let dataPoints = $derived({
		Q1: [{ x: SCALAR * leftScore, y: SCALAR * liberalScore, label: delegate.name, color: color }]
	});
</script>

<svelte:window bind:innerWidth={windowWidth} />

<div class={isMobile ? 'w-40' : 'w-60'}>
	<QuadrantChart
		{dataPoints}
		xLabels={['KAPITALISTISCH', 'SOZIALISTISCH']}
		yLabels={['AUTORITÄR', 'LIBERTÄR']}
	/>
</div>
