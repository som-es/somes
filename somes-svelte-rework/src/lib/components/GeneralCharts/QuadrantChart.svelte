<script lang="ts">
    import { modeCurrent } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';

    export let width: number;
    export let height: number;

    interface DataPoint {
        x: number;
        y: number;
        label: string;
    }

    export let dataPoints: { [key: string]: DataPoint[] } = {
        Q1: [
        { x: 60, y: 70, label: 'A' },
        { x: 80, y: 90, label: 'B' },
        { x: 100, y: 85, label: 'C' },
        ],
        Q2: [
        { x: -60, y: 70, label: 'D' },
        { x: -80, y: 90, label: 'E' },
        { x: -100, y: 85, label: 'F' },
        ],
        Q3: [
        { x: -60, y: -70, label: 'G' },
        { x: -80, y: -90, label: 'H' },
        { x: -100, y: -85, label: 'I' },
        ],
        Q4: [
        { x: 60, y: -70, label: 'J' },
        { x: 80, y: -90, label: 'K' },
        { x: 100, y: -85, label: 'L' },
        ],
    };

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
            quadrant.forEach(({ x, y, label }) => {
                const canvasX = width / 2 + x;
                const canvasY = height / 2 - y;

                ctx.fillStyle = '#3498db';
                ctx.beginPath();
                ctx.arc(canvasX, canvasY, 10, 0, Math.PI * 2);
                ctx.fill();

                ctx.fillStyle = textColor;
                ctx.font = '12px Arial';
                ctx.textAlign = 'center';
                ctx.fillText(label, canvasX, canvasY - 15);
            });
        });
    };

    const drawWithMode = () => {
        const isLightMode = $modeCurrent
        let color = "#FFFFFF"
        if (isLightMode) {
            color = "#000000";
        }
        drawChart(color, color);
    };

    onMount(() => {
        drawWithMode()
    });

    $: if (dataPoints || $modeCurrent) {
        drawWithMode()
    }

</script>

<style>
  canvas {
    border: 2px solid black;
    display: block;
    margin: 0 auto;
  }
</style>

<canvas bind:this={canvas}></canvas>
