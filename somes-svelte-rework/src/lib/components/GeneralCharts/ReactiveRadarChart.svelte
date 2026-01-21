<script lang="ts">
	import { onMount } from 'svelte';
	import ApexCharts, { type ApexOptions } from 'apexcharts';
	import type { ChartData } from '$lib/types';
	export let chartData: ChartData[];

	export let title: string;

	let chart: ApexCharts | null = null;
	let chartOptions: ApexOptions;

	$: {
		const labels = chartData.map((del) => `${del.label}`);
		const isBlack = $modeCurrent;

		chartOptions = {
			series: [
				{
					name: 'Series 1',
					data: chartData.map((x) => x.data)
				}
			],
			chart: {
				height: 350,
				type: 'radar'
			},
			title: {
				text: 'Basic Radar Chart'
			},
			yaxis: {
				stepSize: 20
			},
			xaxis: {
				categories: labels,
				labels: {
					style: {
						colors: labels.map(() => {
							if (isBlack) {
								return 'black';
							} else {
								return 'white';
							}
						}),
						fontSize: '12px'
					}
				}
			}
		};

		if (chart) {
			chart.updateOptions(chartOptions);
		}
	}

	let chartElement: Element;
	onMount(() => {
		// const chartElement = document.querySelector('#chart');

		if (chartElement) {
			chart = new ApexCharts(chartElement, chartOptions);
			chart.render();
		}

		return () => {
			chart?.destroy();
		};
	});

	$: if (chart && chartOptions) {
		chart.updateOptions(chartOptions);
	}
</script>

<div bind:this={chartElement} class="text-black w-96 h-96"></div>

<style>
</style>
