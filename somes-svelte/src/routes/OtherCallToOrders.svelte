<script lang="ts">
	import { delegates_by_call_to_orders, delegates_by_call_to_orders_and_legis_period } from '$lib/api';
	import { getPartyToColor, partyToColorFn } from '$lib/getPartyToColor';
	import type { DelegateByCallToOrders } from '$lib/types';
    import * as d3 from 'd3';
	import { onMount } from 'svelte';

    let upd: Function = () => {};

    // create 2 data_set

    var data0: {group: string, value: number}[] = [];

    var data1: {group: string, value: number}[] = [
        {group: "A", value: 4},
        {group: "Längere Gruppe", value: 16},
        {group: "C", value: 8}
    ];

    var data2: {group: string, value: number}[] = [
        {group: "A", value: 7},
        {group: "Rendi-Wagner Pamela, Dr., MSc (SPÖ)", value: 1},
        {group: "C", value: 20},
        {group: "D", value: 10}
    ];

    var sliceDelegatesByCallToOrders: DelegateByCallToOrders[] = [];

    const partyToColor = getPartyToColor();

    onMount(async () => {
        const delegatesByCallToOrders = await delegates_by_call_to_orders();

        sliceDelegatesByCallToOrders = delegatesByCallToOrders.slice(0, 10);
        
        sliceDelegatesByCallToOrders.forEach((val) => {
            data0.push({group: val.name, value: val.call_to_order_amount});
        })

        // set the dimensions and margins of the graph
        var margin = {top: 30, right: 30, bottom: 70, left: 30},
            width = 760 - margin.left - margin.right,
            height = 660 - margin.top - margin.bottom;

        // append the svg object to the body of the page
        var svg = d3.select("#chart")
            .append("svg")
                // .attr("width", width + margin.left + margin.right)
                // .attr("height", height + margin.top + margin.bottom)
                .attr("viewBox", `0 0 ${width + margin.left + margin.right} ${height + margin.top + margin.bottom}`)
                .attr("preserveAspectRatio", "xMinYMin meet")
                .classed("svg-content-responsive", true)
            .append("g")
                .attr("transform",
                    "translate(" + margin.left + "," + margin.top + ")");


        var x = d3.scaleLinear()
            .range([0, width])
        var xAxis = svg.append("g") 
            .attr("transform", "translate(0," + height + ")")
        var y = d3.scaleBand()
            .range([ 0, height ])
            .padding(0.1);
        var yAxis = svg.append("g")
            .attr("class", "myYaxis")    

        // A function that create / update the plot for a given variable:
        function update(data: DelegateByCallToOrders[]) {

            // Update the X axis
            y.domain(data.map(function(d) { return d.name + ""; }))
            // y.domain(data.map(function(d) { return ""; }))
            yAxis.call(d3.axisLeft(y))

            // Update the Y axis
            // @ts-ignore
            x.domain([0, d3.max(data, function(d) { return d.call_to_order_amount }) ]);
            xAxis.transition().duration(1000).call(d3.axisBottom(x));
            
            // hide group names / labels
            yAxis.selectAll("text").remove()

            // Create the u variable
            var u = svg.selectAll("rect")
                .data(data)
            
            u.enter()
                .append("rect") // Add a new rect for each new elements
                // @ts-ignore
                .merge(u) // get the already existing elements as well
                .transition() // and apply changes to all of them
                .duration(1000)
                // @ts-ignore
                .attr("y", function(d) { return y(d.name); })
                .attr("x", 0)
                .attr("height", y.bandwidth())
                // .attr("width", function(d) { return height - y(d.value); })
                .attr("width", function(d) { return x(d.call_to_order_amount); })
                .attr("fill", (d) => partyToColorFn(partyToColor, d.party))
        
            
            // If less group in the new dataset, I delete the ones not in use anymore
            u
                .exit()
                .remove()
                
            var texts = svg.selectAll(".data-label")
                .data(data)

            texts.enter()
                .append("text")
                .attr("class", "data-label")
                // @ts-ignore
                .merge(texts)
                // @ts-ignore
                .transition()
                .duration(1000)
                // @ts-ignore
                .attr("y", function(d) { return y(d.name); })
                .attr("x", function(d) { return x(d.call_to_order_amount); })
                .attr("dx", (d) => {
                    if (x(d.call_to_order_amount) < 140) {
                        return +2;
                    } else {
                        return -4;
                    }
                }) // padding-right
                .attr("dy", "1.2em") // vertical-align: middle
                .style("text-anchor", function(d) { return x(d.call_to_order_amount) < 140 ? "start" : "end"; })
                // set text color to white
                .attr("fill", "white")
                .text(function(d) { return d.name + " (" + d.party + ")"; });
        
            texts
                .exit()
                .remove()
        }

        upd = update
        // Initialize the plot with the first dataset
        update(sliceDelegatesByCallToOrders)
    })

    async function updateWithLegisData(period: string) {
        const delegatesByCallToOrders = (await delegates_by_call_to_orders_and_legis_period(period)).slice(0, 10);
        upd(delegatesByCallToOrders);
    }   

    const periods = ["XX", "XXI", "XXII", "XXIII", "XXIV", "XXV", "XXVI", "XXVII"];

</script>

{#each periods as period}
    <button style="background-color: aqua; margin: 3px;" on:click={() => updateWithLegisData(period)}>{period}</button>

{/each}
<button style="background-color: aqua;" on:click={() => upd(sliceDelegatesByCallToOrders)}>All time</button>
<div id="chart"></div>
