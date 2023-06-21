<script lang="ts">
    import * as d3 from 'd3';
	import { onMount } from 'svelte';

    let upd: Function = () => {};

    // create 2 data_set
    var data1: {group: string, value: number}[] = [
        {group: "A", value: 4},
        {group: "B", value: 16},
        {group: "C", value: 8}
    ];

    var data2: {group: string, value: number}[] = [
        {group: "A", value: 7},
        {group: "B", value: 1},
        {group: "C", value: 20},
        {group: "D", value: 10}
    ];

    onMount(() => {

    // set the dimensions and margins of the graph
    var margin = {top: 30, right: 30, bottom: 70, left: 60},
        width = 460 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append the svg object to the body of the page
    var svg = d3.select("#chart")
        .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
        .append("g")
            .attr("transform",
                "translate(" + margin.left + "," + margin.top + ")");

    // Initialize the X axis
    var x = d3.scaleBand()
    .range([ 0, width ])
    .padding(0.2);
    var xAxis = svg.append("g")
    .attr("transform", "translate(0," + height + ")")

    // Initialize the Y axis
    var y = d3.scaleLinear()
    .range([ height, 0]);
    var yAxis = svg.append("g")
    .attr("class", "myYaxis")


    // A function that create / update the plot for a given variable:
    function update(data: {group: string, value: number}[]) {

        // Update the X axis
        x.domain(data.map(function(d) { return d.group; }))
        xAxis.call(d3.axisBottom(x))

        // Update the Y axis
        // @ts-ignore
        y.domain([0, d3.max(data, function(d) { return d.value }) ]);
        yAxis.transition().duration(1000).call(d3.axisLeft(y));

        // Create the u variable
        var u = svg.selectAll("rect")
            .data(data)

        u
            .enter()
            .append("rect") // Add a new rect for each new elements
            // @ts-ignore
            .merge(u) // get the already existing elements as well
            .transition() // and apply changes to all of them
            .duration(1000)
            // @ts-ignore
            .attr("x", function(d) { return x(d.group); })
            .attr("y", function(d) { return y(d.value); })
            .attr("width", x.bandwidth())
            .attr("height", function(d) { return height - y(d.value); })
            .attr("fill", "#69b3a2")

        // If less group in the new dataset, I delete the ones not in use anymore
        u
            .exit()
            .remove()
    }

    upd = update
    // Initialize the plot with the first dataset
    update(data1)
    })

</script>

<button on:click={() => upd(data1)}>data1</button>
<button on:click={() => upd(data2)}>data2</button>
<div id="chart"></div>
