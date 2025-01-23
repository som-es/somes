<script lang="ts">
	import { partyToColor } from "$lib/partyColor";
	import type { Delegate, PoliticalPosition } from "$lib/types";
	import QuadrantChart from "../GeneralCharts/QuadrantChart.svelte";
	import type { DataPoint } from "../GeneralCharts/types";

	export let politicalPosition: PoliticalPosition;	
	export let delegate: Delegate;

	const color = partyToColor(delegate.party)

	const leftScore = politicalPosition.is_not_left - politicalPosition.is_left;
	const liberalScore = politicalPosition.is_not_liberal - politicalPosition.is_liberal;

	let dataPoints: { [key: string]: DataPoint[] } = {
        Q1: [
			{ x: 100 * leftScore, y: 100 * liberalScore, label: delegate.name, color: color },
        ],
	}
	
</script>

<QuadrantChart {dataPoints} xLabels={["RECHTS", "LINKS"]} yLabels={["AUTORITÄR", "LIBERTÄR"]} width={240} height={240} />
