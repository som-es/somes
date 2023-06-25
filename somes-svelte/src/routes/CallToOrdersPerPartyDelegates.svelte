<script lang="ts">
    import { onMount } from "svelte";
    import * as d3 from 'd3';
    import { call_to_orders_per_party_delegates } from "$lib/api";
	import type { CallToOrdersPerPartyDelegate } from "$lib/types";
	import { getPartyToColor } from "$lib/getPartyToColor";
    
    let callToOrdersPerPartyDelegates: CallToOrdersPerPartyDelegate[];
    let chart: HTMLElement;

    onMount(async () => {
        callToOrdersPerPartyDelegates = await call_to_orders_per_party_delegates();

        // set the dimensions and margins of the graph
        const width = 450, height = 450, margin = 40;

        // The radius of the pieplot is half the width or half the height (smallest one). I subtract a bit of margin.
        const radius = Math.min(width, height) / 2 - margin

        const svg= d3.select(chart)
            .append("svg")
                .attr("viewBox", "0 0 " + width + " " + height)
                // .attr("width", width)
                // .attr("height", height)
            .append("g")
                .attr("transform", `translate(${width/2},${height/2})`);

        // let data = new Map<string, number>();
        // let data: [string, number][];
        

        // Create dummy data

        // get color and parties from api


        const partyToColor = getPartyToColor();

        const data: { [key: string]: number } = {};

        partyToColor.forEach((_, key) => {
            data[key] = 0;
        })
        callToOrdersPerPartyDelegates.forEach((val) => {
            data[val.party] = val.ratio
        })

        // Compute the position of each group on the pie:
        const pie = d3.pie<[string, number]>()
            .sort(null) // Do not sort group by size
            .value(d => d[1]);

        const data_ready = pie(Object.entries(data))

        // The arc generator
        const arc = d3.arc<d3.DefaultArcObject, [string, number]>()
            .innerRadius(radius * 0.5)         // This is the size of the donut hole
            .outerRadius(radius * 0.8)

        // Another arc that won't be drawn. Just for labels positioning
        const outerArc = d3.arc()
            .innerRadius(radius * 0.8)
            .outerRadius(radius * 0.8)

        // Build the pie chart: Basically, each part of the pie is a path that we build using the arc function.
        svg
            .selectAll('allSlices')
            .data(data_ready)
            .join('path')
            // @ts-ignore
            .attr('d', arc)
            // @ts-ignore
            .attr('fill', d => partyToColor.get(d.data[0]))
            .attr("stroke", "white")
            .style("stroke-width", "2px")
            .style("opacity", 0.7)

        // Add the polylines between chart and labels:
        svg
            .selectAll('allPolylines')
            .data(data_ready)
            .join('polyline')
            .attr("stroke", "currentColor")
            .style("fill", "none")
            .attr("stroke-width", 1)
            // @ts-ignore
            .attr('points', function(d) {
                if (d.data[1] === 0) return;

                // @ts-ignore
                const posA = arc.centroid(d) // line insertion in the slice
                // @ts-ignore
                const posB = outerArc.centroid(d) // line break: we use the other arc generator that has been built only for that
                // @ts-ignore
                const posC = outerArc.centroid(d); // Label position = almost the same as posB
                const midangle = d.startAngle + (d.endAngle - d.startAngle) / 2 // we need the angle to see if the X position will be at the extreme right or extreme left
                posC[0] = radius * 0.89 * (midangle < Math.PI ? 1 : -1); // multiply by 1 or -1 to put it on the right or on the left
                return [posA, posB, posC]
            })

        // Add the polylines between chart and labels:
        svg
        .selectAll('allLabels')
        .data(data_ready)
        .join('text')
            .text(d => {
                if (d.data[1] == 0) return "";
                return d.data[0]
            })
            .attr("fill", "currentColor")
            .attr('transform', function(d) {
                // @ts-ignore
                const pos = outerArc.centroid(d);
                const midangle = d.startAngle + (d.endAngle - d.startAngle) / 2
                pos[0] = radius * 0.93 * (midangle < Math.PI ? 1 : -1);
                return `translate(${pos})`;
            })
            .style('text-anchor', function(d) {
                const midangle = d.startAngle + (d.endAngle - d.startAngle) / 2
                return (midangle < Math.PI ? 'start' : 'end')
            })
        })
</script>

<div>
    <div class="chart" bind:this={chart}></div>
</div>

<style>
.chart {
    max-width: 50rem;
}
</style>