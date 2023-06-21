<script lang="ts">
	import { delegates_by_call_to_orders } from "$lib/api";
    import type { DelegateByCallToOrders } from "$lib/types";
	import { onMount } from "svelte";
    import * as d3 from 'd3';
	import { getPartyToColor } from "$lib/getPartyToColor";

    let delegatesByCallToOrders: DelegateByCallToOrders[];
	
    let el: HTMLElement;

    function displayBarChart(delegatesByCallToOrders: DelegateByCallToOrders[], partyToColor: Map<string, string>) {

        // displayBarChart(speakersByHours)
        // const resultsOnlyHours = speakersByHours.map(result => result.hours_spoken).slice(0, 5);
        // var resultsOnlyHours = [30, 86, 168, 281, 303, 365];
        d3.select(el)
			.selectAll("div")
			.data(delegatesByCallToOrders)
			.enter()
			.append("div")
            .style("background-color", function(d) {
                const color = partyToColor.get(d.party);
				return color == null ? "#808080" : color;
			})
            .style("overflow", "hidden")
			.style("width", function(d) {
				return d.call_to_order_amount * 20 + "px";
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
    }

    onMount(async function () {
        delegatesByCallToOrders = await delegates_by_call_to_orders();

        const sliceDelegatesByCallToOrders = delegatesByCallToOrders.slice(0, 10);
        displayBarChart(sliceDelegatesByCallToOrders, getPartyToColor());
		
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
