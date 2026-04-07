<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, PoliticalPosition } from '$lib/types';
	import QuadrantChart from '../../GeneralCharts/QuadrantChart.svelte';
	import type { DataPoint } from '../../GeneralCharts/types';



	let { politicalPosition, 
		delegate, 
		isMobile }: { 
			politicalPosition: PoliticalPosition; 
			delegate: Delegate; 
			isMobile: boolean 
		} = $props();

	let color = $derived(partyToColor(delegate.party));

	let leftScore = $derived(politicalPosition.is_not_left - politicalPosition.is_left);
	let liberalScore = $derived(politicalPosition.is_not_liberal - politicalPosition.is_liberal);

	const SCALAR = 20;
	let dataPoints = $derived({
		Q1: [{ x: SCALAR * leftScore, y: SCALAR * liberalScore, label: delegate.name, color: color }]
	});
</script>

<div class={isMobile ? 'w-45' : 'w-65'}>
	<QuadrantChart
		{dataPoints}
		xLabels={['KAPITALISTISCH', 'SOZIALISTISCH']}
		yLabels={['AUTORITÄR', 'LIBERTÄR']}
	/>
</div>
