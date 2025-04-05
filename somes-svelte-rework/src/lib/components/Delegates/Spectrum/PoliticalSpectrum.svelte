<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, PoliticalPosition } from '$lib/types';
	import QuadrantChart from '../../GeneralCharts/QuadrantChart.svelte';
	import type { DataPoint } from '../../GeneralCharts/types';

	export let politicalPosition: PoliticalPosition;
	export let delegate: Delegate;

	$: color = partyToColor(delegate.party);

	$: leftScore = politicalPosition.is_not_left - politicalPosition.is_left;
	$: liberalScore = politicalPosition.is_not_liberal - politicalPosition.is_liberal;
	console.log(leftScore);

	$: dataPoints = {
		Q1: [{ x: 1000 * leftScore, y: 1000 * liberalScore, label: delegate.name, color: color }]
	};
</script>

<QuadrantChart
	{dataPoints}
	xLabels={['RECHTS', 'LINKS']}
	yLabels={['AUTORITÄR', 'LIBERTÄR']}
	width={240}
	height={240}
/>
