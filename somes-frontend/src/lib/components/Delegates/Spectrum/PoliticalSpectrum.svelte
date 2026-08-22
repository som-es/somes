<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, PoliticalScore } from '$lib/types';
	import QuadrantChart from '../../GeneralCharts/QuadrantChart.svelte';
	import type { DataPoint } from '../../GeneralCharts/types';

	let {
		politicalPosition,
		delegate
	}: { politicalPosition: PoliticalScore; delegate: Delegate } = $props();

	let windowWidth = $state(700);
	let isMobile = $derived(windowWidth < 640);

	let color = $derived(partyToColor(delegate.party));

	let leftScore = $derived(politicalPosition.capitalist - politicalPosition.socialist);
	let liberalScore = $derived(politicalPosition.authoritarian - politicalPosition.liberal);

	const SCALAR = 800;
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
