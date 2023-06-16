<script lang="ts">
	import { speakers_by_hours } from "$lib/api";
    import type { SpeakerByHours } from "$lib/types";
	import { onMount } from "svelte";
    import * as d3 from 'd3';

    let speakersByHours: SpeakerByHours[];
	
    let el: HTMLElement;

    onMount(async function () {
        speakersByHours = await speakers_by_hours();

        const sliceSpeakersByHours = speakersByHours.slice(0, 10);

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
            .style("height", function(d) {
				return "50px";
			})
            
			.text(function(d) {
				return d.name + " (" + d.party + ")";
			});
    });

    let partyToColor = new Map<string, string>();
    partyToColor.set("SPÖ",  "#E31E2D");
    partyToColor.set("ÖVP",  "#62C3D0");
    partyToColor.set("FPÖ",  "#0052FB");
    partyToColor.set("GRÜNE", "#69B12E");
    partyToColor.set("NEOS", "#E3257B");

    // Set up chart dimensions and styling
    const chartWidth = 1000;
    const chartHeight = 500;

    const displayBarChart = (results: SpeakerByHours[]) => {

        const resultsOnlyHours = results.map(result => result.hours_spoken);

        const length = 10;

        // Get the canvas element
        const canvas = document.getElementById('chart') as HTMLCanvasElement;
        if (canvas == null)  {
            return;
        }
        const context = canvas.getContext('2d');

        if (context == null) {
            return;
        }

        const barPadding = 10;
        const barWidth = (chartWidth / length) - barPadding;

        // Calculate the maximum value in the results
        const maxresultsValue = Math.max(...resultsOnlyHours);

        // Set the chart's background color
        context.fillStyle = '#f2f2f2';
        context.fillRect(0, 0, chartWidth, chartHeight);
        
        // Draw the bars
        for (let i = 0; i < length; i++) {
            const barHeight = (results[i].hours_spoken / maxresultsValue) * chartHeight;
            const x = i * (barWidth + barPadding);
            const y = chartHeight - barHeight;

            // Set the bar color
            

            let color = partyToColor.get(results[i].party)
            if (color == null) {
                // grey
                color = "#808080"
            }

            context.fillStyle = color;
            context.fillRect(x, y, barWidth, barHeight);
            
            // draw the party name
            context.fillStyle = '#000';
            context.font = '12px Arial';
            context.textAlign = 'center';
            
            // rotate text by 90 degrees
            context.translate(1, 1);
            context.rotate(-Math.PI / 2);

            context.fillText(results[i].name, x + (barWidth / 2), chartHeight - 5);
            
            // rotate text back to normal
            context.rotate(Math.PI / 2);
            context.translate(-1, -(1));            
        }    
    }
    	
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
