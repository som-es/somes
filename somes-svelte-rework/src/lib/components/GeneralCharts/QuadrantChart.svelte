<script lang="ts">
    import { modeCurrent } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';
	import type { DataPoint } from './types';

    export let width: number;
    export let height: number;
 

    export let dataPoints: { [key: string]: DataPoint[] };
    // export let dataPoints: { [key: string]: DataPoint[] } = {
    //     Q1: [
    //     { x: 60, y: 30, label: 'A', color: "" },
    //     { x: 80, y: 50, label: 'B' },
    //     { x: 100, y: 35, label: 'C' },
    //     ],
    //     Q2: [
    //     { x: -60, y: 30, label: 'D' },
    //     { x: -80, y: 45, label: 'E' },
    //     { x: -100, y: 35, label: 'F' },
    //     ],
    //     Q3: [
    //     { x: -60, y: -30, label: 'G' },
    //     { x: -80, y: -20, label: 'H' },
    //     { x: -100, y: -60, label: 'I' },
    //     ],
    //     Q4: [
    //     { x: 60, y: -40, label: 'J' },
    //     { x: 80, y: -50, label: 'K' },
    //     { x: 100, y: -55, label: 'L' },
    //     ],
    // };

    let canvas: HTMLCanvasElement;

    const drawChart = (textColor: string, lineColor: string) => {
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const width = canvas.width;
        const height = canvas.height;

        ctx.clearRect(0, 0, width, height);

        ctx.strokeStyle = lineColor;
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(width / 2, 0);
        ctx.lineTo(width / 2, height);
        ctx.moveTo(0, height / 2);
        ctx.lineTo(width, height / 2);
        ctx.stroke();

        Object.values(dataPoints).forEach((quadrant) => {
            console.log(quadrant)
            quadrant.forEach(({ x, y, label, color }) => {
                const canvasX = width / 2 + x;
                const canvasY = height / 2 - y;

                ctx.fillStyle = color;
                ctx.beginPath();
                ctx.arc(canvasX, canvasY, 8, 0, Math.PI * 2);
                ctx.fill();

                ctx.fillStyle = textColor;
                ctx.font = '11px Arial';
                ctx.textAlign = 'center';
                ctx.fillText(label, canvasX, canvasY - 15);
            });
        });
    };

    const drawWithMode = () => {
        const isLightMode = $modeCurrent
        let borderColor = "#FFFFFF2b"
        let textColor = "#FFFFFF"
        if (isLightMode) {
            borderColor = "#0000002b";
            textColor = "#000000"
        }
        drawChart(textColor, borderColor);
    };

    onMount(() => {
        drawWithMode()
    });

    $: if (dataPoints || $modeCurrent) {
        drawWithMode()
    }

</script>

<canvas class="border-black dark:border-white" bind:this={canvas}></canvas>

<style>
    canvas {
        border: 3px solid;
    }
</style>

