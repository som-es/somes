<script lang="ts">
	import { speakers_by_hours } from "$lib/api";
    import type { SpeakerByHours } from "$lib/types";
	import { onMount } from "svelte";
    import * as d3 from 'd3';
	import { getPartyToColor } from "$lib/getPartyToColor";

    let speakersByHours: SpeakerByHours[];
	
    let el: HTMLElement;

    onMount(async function () {
        speakersByHours = await speakers_by_hours();

        const sliceSpeakersByHours = speakersByHours.slice(0, 10);

		const partyToColor = getPartyToColor();

        // displayBarChart(speakersByHours)
        // const resultsOnlyHours = speakersByHours.map(result => result.hours_spoken).slice(0, 5);
        // var resultsOnlyHours = [30, 86, 168, 281, 303, 365];
		d3.select(el)
			.selectAll("div")
			.data(sliceSpeakersByHours)
			.enter()
			.append("div")
            .style("background-color", function(d) {
                const color = partyToColor.get(d.party);
				return color == null ? "#808080" : color;
			})
			.style("width", function(d) {
				return d.hours_spoken * 20 + "px";
			})
            .style("max-width", function(d) {
				return "100%";
			})
            .style("height", function(d) {
				return "50px";
			})
			.text(function(d) {
				return d.name + " (" + d.party + ")";
			});
    });
    	
</script>

<div>
    <!-- <canvas style="" id="chart" width={chartWidth} height={chartHeight}></canvas> -->
    <div class="chart" bind:this={el}></div>
</div>

<style>
	.chart :global(div) {
		font: 10px sans-serif;
		background-color: steelblue;
		text-align: right;
		padding: 3px;
		margin: 1px;
		color: white;
	}
</style>
