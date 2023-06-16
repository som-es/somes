<script lang="ts">
	import { speakers_by_hours } from "$lib/api";
    import type { SpeakerByHours } from "$lib/types";
	import { onMount } from "svelte";
    // import * as d3 from 'd3';

    let speakersByHours: SpeakerByHours[];

    onMount(async function () {
        speakersByHours = await speakers_by_hours();

    });

    let partyToColor = new Map<string, string>();
    partyToColor.set("SPÖ", "#f48992");
    partyToColor.set("ÖVP", "#567a7f");
    partyToColor.set("FPÖ", "#354776");
    partyToColor.set("GRÜNE", "#53723");
    partyToColor.set("NEOS", "#934b6c");

    const displayBarChart = (results: SpeakerByHours[]) => {

        const resultsOnlyHours = results.map(result => result.hours_spoken);

        // Get the canvas element
        const canvas = document.getElementById('chart') as HTMLCanvasElement;
        if (canvas == null)  {
            return;
        }
        const context = canvas.getContext('2d');

        if (context == null) {
            return;
        }

        // Set up chart dimensions and styling
        const chartWidth = 300;
        const chartHeight = 200;
        const barPadding = 10;
        const barWidth = (chartWidth / results.length) - barPadding;

        // Calculate the maximum value in the results
        const maxresultsValue = Math.max(...resultsOnlyHours);

        // Set the chart's background color
        context.fillStyle = '#f2f2f2';
        context.fillRect(0, 0, chartWidth, chartHeight);
        
        // Draw the bars
        for (let i = 0; i < 5; i++) {
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
            context.fillText(results[i].name, x + (barWidth / 2), chartHeight - 5);
        }    
}
</script>

<div>
    <canvas id="chart" width="300" height="300"></canvas>
</div>
